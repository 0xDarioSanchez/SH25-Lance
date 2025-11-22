use soroban_sdk::{Address, Env, String};

use crate::methods::balance::*;
use crate::storage::{constants::*, error::Error, service::*, service_status::ServiceStatus};
use crate::events::event::created_service;

use crate::methods::token::token_transfer;

/*
 * Create a new service from the employee, set the status to created and store it.
*/
pub fn create_service(
    env: &Env,
    creator: Address,
    employer: Address,
    id: u32,
    duration: u64, // In days
    metadata: Option<String>,
    milestone_payment: i128,
) -> Result<Service, Error> {
    creator.require_auth();

    if duration < 1 {
        return Err(Error::InvalidDuration);
    }

    let duration_in_seconds = duration * SECONDS_PER_DAY; // convert days to seconds

    let service = Service {
        id,
        metadata,
        employee: creator.clone(),
        employer,
        duration: duration_in_seconds,
        started_moment: 0,
        status: ServiceStatus::CREATED,
        current_milestone: 1,
        milestone_payment,
    };

    set_service(env, id, service.clone());

    created_service(env, &creator, &id);
    Ok(service)
}

/* Accept the service from the employer, transfer the first milestone payment
 * to the contract and set the start time.
*/
pub fn accept_service(env: &Env, employer: Address, id: u32) -> Result<Service, Error> {
    employer.require_auth();

    let mut service = get_service(env, id)?;

    if service.employer != employer {
        return Err(Error::NotAuthorized);
    }

    service.started_moment = env.ledger().timestamp();

    // let token = crate::methods::token::get_token(env)?;

    token_transfer(
        env,
        &employer,
        &env.current_contract_address(),
        &service.milestone_payment,
    )?;

    service.status = ServiceStatus::ACCEPTED;

    set_service(env, id, service.clone());

    Ok(service)
}

/*
 *Approve the milestone from the employer, increase the employee balance accoding
  the milestone payment and let the service as status.
*/
pub fn approve_milestone(env: &Env, employer: Address, id: u32) -> Result<Service, Error> {
    employer.require_auth();

    let mut service = get_service(env, id)?;

    if service.employer != employer {
        return Err(Error::NotAuthorized);
    }

    // if service.started_moment + service.duration < env.ledger().timestamp() {
    //     return Err(Error::InsufficientTime);
    // }

    let employee_balance = get_balance(env, &service.employee);
    set_balance(
        env,
        &service.employee,
        employee_balance + service.milestone_payment,
    );

    service.current_milestone += 1;
    service.status = ServiceStatus::WAITING;

    set_service(env, id, service.clone());

    Ok(service)
}

/* Approve the service from the employer, transfer the last milestone payment
 * to the contract and remove the service.
*/
pub fn approve_service(env: &Env, employer: Address, id: u32) -> Result<Service, Error> {
    employer.require_auth();

    let service = get_service(env, id)?;

    if service.employer != employer {
        return Err(Error::NotAuthorized);
    }

    // if service.started_moment + service.duration < env.ledger().timestamp() {
    //     return Err(Error::InsufficientTime);
    // }

    let employee_balance = get_balance(env, &service.employee);
    set_balance(
        env,
        &service.employee,
        employee_balance + service.milestone_payment,
    );

    remove_service(env, id);

    Ok(service)
}

/*
 * Add a new milestone from the employee, set a new duration and milestone payment.
*/
pub fn add_milestone(
    env: &Env,
    employee: Address,
    id: u32,
    duration: u64, // in days
    payment: i128,
) -> Result<Service, Error> {
    employee.require_auth();

    let mut service = get_service(env, id)?;

    if service.employee != employee {
        return Err(Error::NotAuthorized);
    }

    if service.status != ServiceStatus::WAITING {
        return Err(Error::InvalidStatus);
    }

    if service.duration != 0 || service.milestone_payment != 0 {
        return Err(Error::InvalidService);
    }

    let duration_in_seconds = duration * SECONDS_PER_DAY; // convert days to seconds

    service.duration += duration_in_seconds;
    service.milestone_payment += payment;

    set_service(env, id, service.clone());

    //TODO add event

    Ok(service)
}

/*
 * Redeem the balance of the employee, transfer the amount to his address and set the balance to zero.
*/
pub fn redeem(env: &Env, employee: Address) -> Result<i128, Error> {
    employee.require_auth();

    let balance = get_balance(env, &employee);

    if balance == 0 {
        return Err(Error::BalanceIsZero);
    }

    set_balance(env, &employee, 0);

    token_transfer(env, &env.current_contract_address(), &employee, &balance)?;

    Ok(balance)
}


//     let converted_amount = balance * conversion_rate;

//     set_balance(env, &employee, 0);

//     token_transfer(
//         env,
//         &env.current_contract_address(),
//         &employee,
//         &converted_amount
//     )?;

//     Ok(converted_amount)
// }
