DROP TABLE IF EXISTS courses CASCADE;
DROP TABLE IF EXISTS tutors;

CREATE TABLE tutors
(
    tutor_id SERIAL PRIMARY KEY,
    tutor_name VARCHAR(200) NOT NULL,
    tutor_pic_url VARCHAR(200) NOT NULL,
    tutor_profile VARCHAR(2000) NOT NULL
);

CREATE TABLE courses
(
    course_id SERIAL PRIMARY KEY,
    tutor_id INT NOT NULL,
    course_name VARCHAR(140) NOT NULL,
    course_description VARCHAR(2000),
    course_format VARCHAR(30),
    course_structure VARCHAR(200),
    course_duration VARCHAR(30),
    course_price INT,
    course_language VARCHAR(30),
    course_level VARCHAR(30),
    posted_time TIMESTAMP default now(),
    CONSTRAINT fk_tutor
    FOREIGN KEY(tutor_id)
        REFERENCES tutors(tutor_id)
);

INSERT INTO  tutors(tutor_id, tutor_name, tutor_pic_url, tutor_profile)
VALUES(DEFAULT,'Merlene','http://s3.amazon.aws.com/pic1','Merlene is an experienced finance professional');

INSERT INTO tutors(tutor_id, tutor_name, tutor_pic_url, tutor_profile)
VALUES(DEFAULT,'Frank','http://s3.amazon.aws.com/pic2','Frank is an expert nuclear engineer');

INSERT INTO courses
    (course_id, tutor_id, course_name, posted_time)
VALUES(DEFAULT, 1, 'First course', '2020-12-17 05:40:00');

INSERT INTO courses
    (course_id, tutor_id, course_name, posted_time)
VALUES(DEFAULT, 1, 'Second course', '2020-12-17 05:45:00');