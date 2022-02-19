DROP TABLE IF EXISTS users;

CREATE TABLE users
(
    username varchar(20) PRIMARY KEY,
    tutor_id INT NOT NULL,
    user_password CHAR(100) NOT NULL
);