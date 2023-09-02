

create schema if not exists cashflow;

create table if not exists cashflow.movement (
    id uuid primary key,
    account_id uuid not null,
    type varchar (6),
    event_id uuid not null,
    previous_balance numeric not null,
    balance numeric not null,
    description varchar not null,
    created_at timestamp with time zone not null
);
create index if not exists idx_movement_id on cashflow.movement (id);
create index if not exists idx_event_id on cashflow.movement (event_id);