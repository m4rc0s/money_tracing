    SET time zone 'America/Sao_Paulo';

    create schema auth
        create table if not exists user_login (
            id uuid not null primary key,
            email varchar(256) not null unique,
            password varchar(256) not null,
            created_at timestamp with time zone not null,
            updated_at timestamp with time zone not null
        )
        create index if not exists idx_user_login_id on user_login (id)

        create table if not exists user_profile (
            id uuid not null primary key,
            cpf varchar (11),
            first_name varchar(70),
            last_name varchar (70),
            email varchar (256),
            user_id uuid not null,
            foreign key (email) references user_login (email),
            foreign key (user_id) references user_login (id)
        )
        create index if not exists idx_user_profile_id on user_profile (id);


    create schema cashflow
        create table if not exists account (
            id uuid not null primary key,
            code varchar(120) not null,
            created_at timestamp with time zone not null,
            updated_at timestamp with time zone not null
        )
        create index if not exists idx_account_id on account (id)
        create index if not exists idx_account_code on account (code)

        create table if not exists debit (
            id uuid not null primary key,
            amount numeric not null,
            account_id uuid not null,
            foreign key (account_id) references account(id),
            created_at timestamp with time zone not null
        )
        create index if not exists idx_debit_id on debit (id)
        
        create table if not exists credit (
            id uuid not null primary key,
            amount numeric NOT NULL,
            account_id uuid not null,
            foreign key (account_id) references account(id),
            created_at timestamp with time zone not null
        )
        create index if not exists idx_credit_id on credit (id);
