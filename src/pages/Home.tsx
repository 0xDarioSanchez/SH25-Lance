import React from "react";
import { Code, Layout, Text } from "@stellar/design-system";
// import { GuessTheNumber } from "../components/GuessTheNumber";

const Home: React.FC = () => (
  <Layout.Content>
    <Layout.Inset>
      <Text as="h1" size="xl">
        Lance Protocol – ZK Voting Integration
      </Text>

      <Text as="p" size="md">
        In this hackathon, we’re focusing entirely on the core smart contract behind 
        <Code size="md">Lance Protocol</Code>. You can explore the current implementation 
        inside the <Code size="md">contracts/</Code> directory — this is where the 
        dispute resolution flow lives in its initial Proof of Concept form.
      </Text>

      <Text as="p" size="md">
        The goal is to refactor this contract to give it a clean and modular structure, 
        preparing the base layer for features like private judge voting. Running 
        <Code size="md">npm run dev</Code> (powered by <Code size="md">stellar scaffold watch</Code>) 
        allows you to see how the contract compiles and how the updated functions are 
        generated as we iterate.
      </Text>

      <Text as="p" size="md">
        We’re also introducing the foundation for a Zero-Knowledge (ZK)–based voting 
        mechanism. All updates and iterations will be pushed to 
        {" "}
        <a href="https://github.com/0xDarioSanchez/SH25-Lance" 
          target="_blank" 
          style={{ textDecoration: "none" }}>
          <Code size="md">github.com/0xDarioSanchez/SH25-Lance</Code>
        </a>, 
        so you can follow the evolution of the contract and the development of the 
        voting model in real time.
      </Text>

    </Layout.Inset>
  </Layout.Content>
);

export default Home;
