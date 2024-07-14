<!--
Hey, thanks for using the awesome-readme-template template.  
If you have any enhancements, then fork this project and create a pull request 
or just open an issue with the label "enhancement".

Don't forget to give this project a star for additional support ;)
Maybe you can mention me or this repo in the acknowledgements too
-->
<div align="center">

  <h1>SwapX</h1>
  
  <p>
    ETHGlobal Hackathon Submission | Brussels 2024 
  </p>
  
  

   
<h4>
    <a href="https://github.com/Louis3797/awesome-readme-template/">View Demo</a>
  <span> · </span>
    <a href="https://github.com/Louis3797/awesome-readme-template">Documentation</a>
  <span> · </span>
    <a href="https://github.com/Louis3797/awesome-readme-template/issues/">Report Bug</a>
  <span> · </span>
    <a href="https://github.com/Louis3797/awesome-readme-template/issues/">Request Feature</a>
  </h4>
</div>

<br />

<!-- Table of Contents -->
# Table of Contents

- [About the Project](#about-the-project)
  * [Key Features](#key-features)
  * [Tech Stack](#tech-stack)
- [Getting Started](#getting-started)
  * [Prerequisites](#prerequisites)
  * [Installation](#installation)
  * [Running Tests](#running-tests)
  * [Deployment](#deployment)
  

<!-- About the Project -->
## About the Project
SwapX is an innovative decentralized trading solution designed to incentivize traders with reduced fees on Uniswap V4. By leveraging custom UniswapV4 hooks and zero-knowledge proofs, SwapX provides a seamless and cost-efficient trading experience. Our project aims to enhance trader engagement and optimize transaction costs through a secure and transparent mechanism.

<!-- Screenshots -->
### Key Features 
SwapX integrates custom hooks into the Uniswap V4 protocol to dynamically adjust trading fees based on predefined conditions. These hooks allow for:

- Dynamic Fee Adjustment: Traders receive reduced fees based on trading volume, activity, and other metrics.
- Enhanced Trading Experience: By customizing fee structures, traders can enjoy a more cost-effective trading environment.

Utilizing zero-knowledge proofs (ZKPs), SwapX ensures the integrity and security of transactions without exposing sensitive information. Key benefits include:

- Proof of Computation: Verify blockchain history and transaction computations efficiently and securely.

We used TheGraph for faster data indexing for ultimate processing.  


<!-- TechStack -->
### Tech Stack

<details>
  <summary>Zero Knowledge</summary>
  <ul>
    <li><a href="https://www.typescriptlang.org/">Axiom</a></li>
    <li><a href="https://nextjs.org/">Circom</a></li>
    <li><a href="https://reactjs.org/">Typescript</a></li>
  </ul>
</details>

<details>
  <summary>Backend</summary>
  <ul>
    <li><a href="https://www.typescriptlang.org/">Solidity</a></li>
    <li><a href="https://expressjs.com/">Rust</a></li>
    <li><a href="https://www.apollographql.com/">Docker</a></li>
    <li><a href="https://graphql.org/">GraphQL</a></li>
  </ul>
</details>



<!-- Getting Started -->
## Getting Started

<!-- Prerequisites -->
### Prerequisites

This project uses Foundry for package management

<!-- Installation -->
### Installation

Install dependencies with forge

```bash
  forge install
  forge build
```
   
<!-- Running Tests -->
### Running Tests

To run tests, run the following command

```bash
  forge test
```


<!-- Deployment -->
### Deployment

To deploy this project run

```bash
  ./deploy-all.sh
```



