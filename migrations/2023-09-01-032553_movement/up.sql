-- Your SQL goes here
create table if not exists movement (
    id UUID not null primary key,
    amount NUMERIC not null,
    description VARCHAR,
    effect VARCHAR (6),
    account_id UUID not null
);

create index if not exists idx_movement_account_id on movement (account_id)