class Block {
  constructor(data, hash, lastHash) {
      this.data = data;
      this.hash = hash;
      this.lastHash = lastHash;
  }
}

function lightningHash(data) {
  return data + "*superSeguro*";
}

class Blockchain {
  constructor() {
      const genesis = new Block("gen-data", "gen-hash", "gen-lastHash");
      this.chain = [genesis];
  }

  addBlock(data) {
      const lastBlock = this.chain[this.chain.length - 1];
      const lastHash = lastBlock.hash;
      const hash = lightningHash(data + lastHash);
      const block = new Block(data, hash, lastHash);
      this.chain.push(block);
  }
}

// Uso da Blockchain
const blockchainPoggers = new Blockchain();
blockchainPoggers.addBlock("one");
blockchainPoggers.addBlock("two");
blockchainPoggers.addBlock("three");

for (const block of blockchainPoggers.chain) {
  console.log(`Data: ${block.data}, Hash: ${block.hash}, LastHash: ${block.lastHash}`);
  console.log("---------------------------------------");
}
