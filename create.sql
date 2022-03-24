DROP TABLE IF EXISTS foo;
CREATE TABLE foo (
    memo varchar,
    import_ts timestamp default now(),
    import_tz timestamp with time zone default now()
);

insert into foo(memo) values('Theo is sneezy but great!');
