-- Initial schema for Money Road.
-- The conceptual graph is the domain model; this physical schema is a starting
-- point and evolves with route scale/query patterns.

CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Providers: payment participants.
CREATE TABLE providers (
    id                 UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name               TEXT NOT NULL,
    markets            TEXT[] NOT NULL DEFAULT '{}',
    capabilities       TEXT[] NOT NULL DEFAULT '{}',
    regulatory_context TEXT,
    created_at         TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- Corridors: origin/destination market pairs.
CREATE TABLE corridors (
    id                  UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    origin_country      TEXT NOT NULL,
    destination_country TEXT NOT NULL,
    source_currency     TEXT NOT NULL,
    destination_currency TEXT NOT NULL,
    UNIQUE (origin_country, destination_country, source_currency, destination_currency)
);

-- Edges: one executable movement/transformation.
CREATE TABLE edges (
    id               UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    corridor_id      UUID NOT NULL REFERENCES corridors(id),
    provider_id      UUID NOT NULL REFERENCES providers(id),
    sequence         INT  NOT NULL,
    rail             TEXT NOT NULL,
    from_node        TEXT NOT NULL,
    to_node          TEXT NOT NULL,
    input_asset      TEXT NOT NULL,
    output_asset     TEXT NOT NULL,
    fee              DOUBLE PRECISION,
    fx_rate          DOUBLE PRECISION,
    min_amount       DOUBLE PRECISION,
    max_amount       DOUBLE PRECISION,
    duration_min_min INT,
    duration_min_max INT,
    status           TEXT NOT NULL DEFAULT 'pending_verification',
    created_at       TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at       TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- Evidence: time-dependent claims with a source and validity window.
CREATE TABLE evidence (
    id           UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    claim        TEXT NOT NULL,
    value        JSONB NOT NULL,
    source       TEXT NOT NULL,
    state        TEXT NOT NULL,
    observed_at  TIMESTAMPTZ NOT NULL,
    expires_at   TIMESTAMPTZ NOT NULL,
    confidence   DOUBLE PRECISION NOT NULL DEFAULT 0.0,
    jurisdiction TEXT,
    conditions   TEXT
);

-- Edges may reference multiple evidence items.
CREATE TABLE edge_evidence (
    edge_id     UUID NOT NULL REFERENCES edges(id) ON DELETE CASCADE,
    evidence_id UUID NOT NULL REFERENCES evidence(id) ON DELETE CASCADE,
    PRIMARY KEY (edge_id, evidence_id)
);

-- Routes: ordered set of edges (the product object).
CREATE TABLE routes (
    id                         UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    corridor_id                UUID NOT NULL REFERENCES corridors(id),
    status                     TEXT NOT NULL DEFAULT 'draft',
    input_amount               DOUBLE PRECISION NOT NULL,
    source_currency            TEXT NOT NULL,
    destination_currency       TEXT NOT NULL,
    expected_sender_spend      DOUBLE PRECISION,
    expected_recipient_amount  DOUBLE PRECISION,
    expected_duration_min      INT,
    expected_duration_max      INT,
    deadline_buffer_seconds    BIGINT,
    confidence                 DOUBLE PRECISION,
    created_at                 TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- Route <-> edge linkage preserving order.
CREATE TABLE route_edges (
    route_id    UUID NOT NULL REFERENCES routes(id) ON DELETE CASCADE,
    edge_id     UUID NOT NULL REFERENCES edges(id),
    sequence    INT  NOT NULL,
    PRIMARY KEY (route_id, edge_id)
);

-- Observations: what actually happened after execution.
CREATE TABLE observations (
    id          UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    route_id    UUID NOT NULL REFERENCES routes(id),
    actual_cost DOUBLE PRECISION,
    actual_time_sec INT,
    succeeded   BOOLEAN NOT NULL,
    notes       TEXT,
    recorded_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- Failure modes: known ways a route fails.
CREATE TABLE failure_modes (
    id         UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    route_id   UUID NOT NULL REFERENCES routes(id),
    trigger    TEXT NOT NULL,
    mitigation TEXT,
    uncertainty BOOLEAN NOT NULL DEFAULT true
);

CREATE INDEX idx_edges_corridor ON edges(corridor_id);
CREATE INDEX idx_routes_corridor ON routes(corridor_id);
CREATE INDEX idx_evidence_expiry ON evidence(expires_at);
