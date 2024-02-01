CREATE DATABASE traffic_data;

CREATE TABLE IF NOT EXISTS traffic_data (
    id SERIAL PRIMARY KEY NOT NULL,
    source TEXT NOT NULL,
    category TEXT NOT NULL,
    device TEXT NOT NULL,
    objects_count INT NOT NULL,
    vehicles_count INT NOT NULL,
    "timestamp" TIMESTAMP WITH TIME ZONE NOT NULL,
);