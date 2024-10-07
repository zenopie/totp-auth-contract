
## Secret Network Smart Contract for Fund Distribution by Voting Weight

### User Focused Summary

Welcome to the Secret Network Smart Contract repository for fund distribution by voting weight! This smart contract allows users to stake their ERTH tokens and participate in the allocation of funds based on a weighted voting system. Here's a quick overview of what it does:

1. **Stake ERTH Tokens**: Stake your ERTH tokens to gain voting power.
2. **Allocate Funds**: Enter your preferred fund allocation percentages, which must total 100%.
3. **Consensus Mechanism**: Your allocation preferences are averaged with those of other stakers to reach a consensus on fund distribution.
4. **Real-time Updates**: See your preferred split and the weighted average of all stakers’ allocations, updated in real-time as staking amounts change.

By participating in this system, you help ensure that funds are distributed effectively and fairly, rewarding efficient public goods institutions and moving away from ineffective ones.

### Investor Pitch

**Problem Statement**:
Current monopolies over public goods funding often lead to misaligned incentives, creating institutions that can be ineffective and exploitative. Constituents suffer as their needs are not adequately met, and public goods remain suboptimal.

**Solution**:
Our smart contract introduces free-market values and direct democracy into public goods funding. By enabling granular, decentralized decision-making, we reward effective institutions and transition away from ineffective ones. This system reduces harm and offers alternative governance models, minimizing the risk of corruption and misalignment.

**Product Market Fit**:
This governance implementation fits in markets where public goods funding is critical, such as community projects, non-profits, and decentralized autonomous organizations (DAOs). It empowers stakeholders by giving them direct control over fund allocation, ensuring their interests are represented and leading to more effective and accountable public goods institutions.

### Development Deepdive

**Build Process**:
The smart contract was developed on the Secret Network, leveraging its privacy-preserving features to ensure secure and confidential voting. The core components include staking functions, allocation input handling, and consensus calculation.

**Contract/Function Interactions**:
1. **Staking Function**: Users stake their ERTH tokens, which grants them voting power proportional to their staked amount.
2. **Allocation Input**: Users enter their preferred fund allocation percentages, which are stored securely.
3. **Consensus Calculation**: The contract calculates the weighted average of all stakers’ allocations to determine the final fund distribution. This ensures that each staker's vote is proportionate to their stake.

**Design Choices**:
- **Privacy-Preserving**: Built on the Secret Network to ensure that individual votes and preferences remain confidential.
- **Real-time Adjustments**: The staking function is integrated to adjust voting weights dynamically as staking amounts change, ensuring up-to-date consensus.
- **Granular Control**: Allows for detailed and precise allocation preferences, promoting effective fund distribution.

### Usage Instructions

1. **Get Testnet ERTH and Viewing Key**: To use the demo, acquire testnet ERTH and a viewing key from the Stake ERTH page. Ensure you have staked your ERTH tokens.
2. **Enter Allocation Preferences**: Navigate to the allocation page (Governance tab, Deflation Fund) and enter your preferred split in percentages. The total must equal 100%.
3. **View Results**: After reloading the page, you will see your preferred split and the weighted average of all stakers who declared an allocation.

This project aims to revolutionize public goods funding by leveraging decentralized governance and secure, transparent smart contracts. Join us in creating a fairer, more efficient system for all!