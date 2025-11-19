#!/bin/bash
#set -e

echo "        ********************************"
echo -e "\t***** Creating accounts *****..."
echo "        ********************************"
# stellar keys address employee-1
stellar keys generate lance-admin --network testnet --fund
stellar keys generate employee-1 --network testnet --fund
stellar keys generate employee-2 --network testnet --fund
stellar keys generate employer-1 --network testnet --fund
stellar keys generate employer-2 --network testnet --fund
stellar keys generate judge-1 --network testnet --fund
stellar keys generate judge-2 --network testnet --fund
stellar keys generate judge-3 --network testnet --fund

echo "        *********************************"
echo -e "\t***** Accounts generated *****..."
echo "        *********************************"