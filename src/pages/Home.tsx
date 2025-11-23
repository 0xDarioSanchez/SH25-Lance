import React, { useEffect, useState } from "react";
import { useLocation } from "react-router-dom";
import { Code, Layout, Text } from "@stellar/design-system";
import { CreateDisputeButton } from "../components/CreateDisputeButton";
import { RegisterJudgeButton } from "../components/RegisterJudgeButton";
import { AnonymousVotingSetup } from "../components/AnonymousVotingSetup";
import { DisputeList } from "../components/DisputeList";
import { getDisputes } from "../api/disputes";
import type { Dispute } from "../api/types";
import { useWallet } from "../hooks/useWallet";
import { MAINTAINER_ADDRESS } from "../contracts/util";

const Home: React.FC = () => {
  const location = useLocation();
  const { address } = useWallet();
  const [disputes, setDisputes] = useState<Dispute[]>([]);
  const [loading, setLoading] = useState(true);
  
  const isMaintainer = address === MAINTAINER_ADDRESS;

  const loadDisputes = async () => {
    try {
      const data = await getDisputes();
      setDisputes(data);
    } catch (err) {
      console.error("Error loading disputes:", err);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    loadDisputes();
    
    // Reload disputes when window regains focus (user comes back to tab)
    const handleFocus = () => {
      loadDisputes();
    };
    
    window.addEventListener('focus', handleFocus);
    
    return () => {
      window.removeEventListener('focus', handleFocus);
    };
  }, [location]); // Reload when location changes (navigating back to home)

  return (
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
          mechanism. All updates and iterations will be pushed to{" "}
          <a
            href="https://github.com/0xDarioSanchez/SH25-Lance"
            target="_blank"
            style={{ textDecoration: "none" }}
          >
            <Code size="md">github.com/0xDarioSanchez/SH25-Lance</Code>
          </a>
          , so you can follow the evolution of the contract and the development of the 
          voting model in real time.
        </Text>

        {/* Register as Judge Button */}
        {!isMaintainer && (
          <div style={{ marginBottom: "20px" }}>
            <RegisterJudgeButton />
          </div>
        )}

        {/* Anonymous Voting Setup - Only shown to maintainer */}
        {isMaintainer ? (
          <AnonymousVotingSetup projectId={1} />
        ) : address && (
          <div style={{ 
            padding: "16px", 
            backgroundColor: "#fff3cd", 
            borderRadius: "8px", 
            marginTop: "16px",
            marginBottom: "16px",
            border: "1px solid #ffc107"
          }}>
            <Text as="p" size="sm" style={{ color: "#856404" }}>
              ℹ️ <strong>Note:</strong> Anonymous voting setup and dispute execution is restricted to the maintainer.
            </Text>
            <Text as="p" size="xs" style={{ color: "#856404", marginTop: "8px" }}>
              Maintainer: {MAINTAINER_ADDRESS.slice(0, 12)}...{MAINTAINER_ADDRESS.slice(-8)}
            </Text>
          </div>
        )}

        {/* Move CreateDisputeButton here, just above DisputeList */}
        {!isMaintainer && (
          <div style={{ marginBottom: "24px" }}>
            <CreateDisputeButton onCreated={loadDisputes} />
          </div>
        )}

        {/* LA LISTA LEE DESDE EL ESTADO DEL PADRE */}
        <DisputeList
          disputes={disputes}
          loading={loading}
        />
      </Layout.Inset>
    </Layout.Content>
  );
};

export default Home;
