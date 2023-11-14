CREATE TABLE IF NOT EXISTS assets (
  address TEXT NOT NULL,
  chain_id INTEGER NOT NULL,
  name TEXT NOT NULL,
  symbol TEXT NOT NULL,
  decimals INTEGER NOT NULL,
  logo_url TEXT NOT NULL,
  price FLOAT NOT NULL,
  PRIMARY KEY (address, chain_id)
);

CREATE TABLE IF NOT EXISTS pairs (
  address TEXT NOT NULL,
  chain_id INTEGER NOT NULL,
  symbol TEXT NOT NULL,
  decimals INTEGER NOT NULL,
  stable BOOLEAN NOT NULL,
  fee FLOAT NOT NULL,
  total_supply FLOAT NOT NULL,
  reserve0 FLOAT NOT NULL,
  reserve1 FLOAT NOT NULL,
  gauge_address TEXT NOT NULL,
  tvl FLOAT NOT NULL,
  token0_address TEXT NOT NULL,
  token0 json NOT NULL,
  token1_address TEXT NOT NULL,
  token1 json NOT NULL,
  PRIMARY KEY (address, chain_id),
  FOREIGN KEY (token0_address, chain_id) REFERENCES assets (address, chain_id),
  FOREIGN KEY (token1_address, chain_id) REFERENCES assets (address, chain_id)
);

CREATE TABLE IF NOT EXISTS aprs (
  pair_address TEXT NOT NULL,
  token_address TEXT NOT NULL,
  chain_id INTEGER NOT NULL,
  apr FLOAT NULL,
  min_apr FLOAT NULL,
  max_apr FLOAT NULL,
  symbol TEXT NOT NULL,
  logo_url TEXT NOT NULL,
  PRIMARY KEY (pair_address, chain_id, token_address),
  FOREIGN KEY (pair_address, chain_id) REFERENCES pairs (address, chain_id),
  FOREIGN KEY (token_address, chain_id) REFERENCES assets (address, chain_id)
);

CREATE TABLE IF NOT EXISTS gauges (
  address TEXT NOT NULL,
  chain_id INTEGER NOT NULL,
  decimals INTEGER NOT NULL,
  total_supply FLOAT NOT NULL,
  bribe_address TEXT NOT NULL,
  reward FLOAT NOT NULL,
  median_tbv FLOAT NOT NULL,
  min_tbv FLOAT NOT NULL,
  max_tbv FLOAT NOT NULL,
  votes FLOAT NOT NULL,
  min_apr FLOAT NOT NULL,
  max_apr FLOAT NOT NULL,
  pair_address TEXT NOT NULL,
  PRIMARY KEY (address, chain_id),
  FOREIGN KEY (pair_address, chain_id) REFERENCES pairs (address, chain_id)
);

CREATE TABLE IF NOT EXISTS killed_gauges (
  address TEXT NOT NULL,
  chain_id INTEGER NOT NULL,
  decimals INTEGER NOT NULL,
  total_supply FLOAT NOT NULL,
  bribe_address TEXT NOT NULL,
  reward FLOAT NOT NULL,
  rewards INTEGER[] DEFAULT ARRAY[]::INTEGER[],
  median_tbv FLOAT NOT NULL,
  min_tbv FLOAT NOT NULL,
  max_tbv FLOAT NOT NULL,
  votes FLOAT NOT NULL,
  min_apr FLOAT NOT NULL,
  max_apr FLOAT NOT NULL,
  bribes INTEGER[] DEFAULT ARRAY[]::INTEGER[],
  pair_address TEXT NOT NULL,
  PRIMARY KEY (address, chain_id),
  FOREIGN KEY (pair_address, chain_id) REFERENCES pairs (address, chain_id)
);

CREATE TABLE IF NOT EXISTS bribes (
  bribe_address TEXT NOT NULL,
  reward_amount FLOAT NOT NULL,
  chain_id INTEGER NOT NULL,
  token_address TEXT NOT NULL,
  symbol TEXT NOT NULL,
  token_decimals INTEGER NOT NULL,
  logo_url TEXT NOT NULL,
  pair_address TEXT NOT NULL,
  PRIMARY KEY (bribe_address, pair_address, token_address),
  FOREIGN KEY (pair_address, chain_id) REFERENCES pairs (address, chain_id),
  FOREIGN KEY (token_address, chain_id) REFERENCES assets (address, chain_id)
);