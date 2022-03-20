# PV204-MPC-Trusted-Timestamp-Server

## < Tohle jsme my >
[]  []  []

## < jste ty >
    []


MPC Trusted Timestamp Server

* Implement a trusted timestamping server that secures its signing key via multiparty computation
* The trusted timestamping server will  
  – Publish its public key  
  – Provide interface through which users can submit a document for timestamping  
  – Output timestamped documents signed with its private key  
* Use multi-party computation to avoid single point of failure  
  – Distribute the private key shares among multiple servers  
  – Use threshold signing scheme to create the signatures  
* Resources  
  – https://github.com/ZenGo-X/multi-party-ecdsa  
  – https://github.com/isislovecruft/frost-dalek  
