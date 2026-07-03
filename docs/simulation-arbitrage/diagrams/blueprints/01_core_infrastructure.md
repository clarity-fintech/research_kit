# Core Infrastructure Blueprints

**Classification:** Cognitive Architecture Blueprints  
**Legend:** `───` Hardened consensus · `- - -` ML inference / telemetry

---

## Cognitive Architecture Blueprint: Nexus Orchestrator

*Product: clrty-core-nexus · Prompt: [PROMPT_LIBRARY § Nexus](../PROMPT_LIBRARY.md#clrty-core-nexus--nexus-orchestrator)*

```mermaid
%%{init: {"theme":"base","themeVariables":{"darkMode":true,"background":"#020617","primaryColor":"#0f172a","primaryTextColor":"#f8fafc","primaryBorderColor":"#22d3ee","lineColor":"#22d3ee","clusterBkg":"#0f172a","clusterBorder":"#334155"}}}%%
flowchart TB
    subgraph legend [Legend]
        direction LR
        LS["Solid — Governance / Consensus"]
        LD["Dashed — Telemetry / ML"]
    end

    NEXUS(("Nexus Hub<br/>Immutable Root of Trust"))

    subgraph substrate [Substrate Cluster]
        L1[CLRTY_SUBSTRATE<br/>PoC + MVM + token_core]
    end

    subgraph vis [VIS Intelligence]
        NT[neuro_templates_engine]
        SK[Quantum Skills / clrty-cli-core]
    end

    subgraph helix [HELIX Engine]
        HX[helix_engine L0.5<br/>MIRRA matching]
    end

    subgraph ops [Operator CLI]
        CLI[clarity-cli / clrty-api]
    end

    NEXUS ==>|governance| L1
    NEXUS ==>|governance| HX
    NEXUS ==>|governance| CLI
    L1 -.->|telemetry| NEXUS
    NT -.->|inference feed| NEXUS
    HX -.->|fill telemetry| NEXUS
    CLI -.->|operator events| NEXUS
    SK --> NT
    HX --> L1
    CLI --> L1
```

| Flow | Style | Description |
|------|-------|-------------|
| Nexus → clusters | Solid | Stage gates, manifests, integrity battery |
| Clusters → Nexus | Dashed | Compliance reports, SIM100 merkle, red flags |

**Repo:** `manifests/nexus_modules.json` · `docs/architecture/NEXUS_REPOSITORY.md`

---

## Cognitive Architecture Blueprint: L1 Substrate Topology

*Product: clrty-substrate · Prompt: [PROMPT_LIBRARY § Substrate](../PROMPT_LIBRARY.md#clrty-substrate--l1-deployment-topology)*

```mermaid
%%{init: {"theme":"base","themeVariables":{"darkMode":true,"background":"#020617","primaryColor":"#0f172a","primaryTextColor":"#f8fafc","primaryBorderColor":"#22d3ee","lineColor":"#22d3ee","clusterBkg":"#0f172a","clusterBorder":"#334155"}}}%%
flowchart LR
    subgraph validators [Federated Validator Network]
        V1[Validator A]
        V2[Validator B]
        V3[Validator N…]
    end

    subgraph hsm [HSM Security Layer]
        HSM{{Hardware Security Module}}
        MK[Cold Master Key]
    end

    subgraph attestation [External Attestation Gateway]
        GW[[Attestation Gateway]]
        GK[🛡 Identity Gatekeeper]
    end

    subgraph sync [Deterministic State Synchronization]
        SR[state_manifold<br/>state_root + genesis_seal]
        WORM[WORM audit trail]
    end

    MK ==>|air-gap provision| HSM
    HSM ==>|signed blocks| V1
    HSM ==>|signed blocks| V2
    HSM ==>|signed blocks| V3
    V1 & V2 & V3 ==>|consensus ticks| SR
    GW ==>|KYC attestation blob| GK
    GK ==>|settlement gate| SR
    SR --> WORM
```

**Repo:** `CLRTY_SUBSTRATE/state_manifold/` · `CLRTY_SUBSTRATE/settlement/attestation_ledger.rs`
