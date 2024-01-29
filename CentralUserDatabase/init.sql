
CREATE DATABASE CentralUserDatabase;
\c CentralUserDatabase


-- Ensure the "uuid-ossp" extension is available
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Create the users table with UUID primary key
CREATE TABLE IF NOT EXISTS users (
    id uuid PRIMARY KEY DEFAULT uuid_generate_v4() UNIQUE,
    password_hash text NOT NULL,
    user_name varchar(32) NOT NULL,
    email text NOT NULL UNIQUE,
    salt varchar(16) NOT NULL
);


