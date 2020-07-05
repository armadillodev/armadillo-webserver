create table orgs (
    org_id integer not null primary key,
    name text not null unique
);

create table trailors (
    trailor_id integer not null primary key,
    name text not null,
    location text not null,
    org integer not null,
    foreign key(org) references orgs(org_id)
);

create table users (
    user_id integer not null primary key,
    first_name text not null,
    last_name text,
    org integer not null,
    foreign key(org) references orgs(org_id)
);

create table trailor_logs (
    trailor_log_id integer not null primary key,
    user integer not null ,
    trailor integer not null,
    time_start integer not null,
    time_end integer not null,
    foreign key(user) references orgs(org_id),
    foreign key(trailor) references trailors(trailor_id)
);

create table trailor_data (
    trailor_data_id integer not null primary key,
    trailor integer not null,
    timestamp integer not null,
    temperature integer,
    foreign key(trailor) references trailors(trailor_id)
);


insert into orgs (name) values ("Kai");
insert into orgs (name) values ("energilab");

insert into trailors (name, location, org)
values (
    "Kai's House",
    "Tokyo, Japan",
    (select org_id from orgs where name="Kai")
);
insert into trailors (name, location, org) 
values (
    "Joe's fan",
    "Denver, CO",
    (select org_id from orgs where name="energilab")
);

insert into users (first_name, last_name, org) 
values (
    "Kai",
    "Dewey",
    (select org_id from orgs where name="Kai")
);
insert into users (first_name, org) 
values (
    "Joe",
    (select org_id from orgs where name="energilab")
);
