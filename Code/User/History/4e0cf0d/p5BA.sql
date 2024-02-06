/*
Databases Applied 09
applied9_sql_basic_intermediate.sql

student id: 33109877
student name:Anthony Oon
applied class number:
last modified date:

*/

/* Part A - Retrieving data from a single table */

-- A1. List the full student details for those students who live in Caulfield. Order the output by student id.
SELECT
    *
FROM
    uni.student
WHERE
    lower(stuaddress) LIKE '%caulfield%'
ORDER BY
    stuid;

-- A2

SELECT
    *
FROM
    uni.unit
WHERE
    REGEXP_LIKE (lower(unitcode), '^fit1...')
ORDER BY
    unitcode;

-- A3 List the student's id, surname, first name and address for those students who have a surname starting with the letter S and first name which contains the letter i. Order the output by student id.
SELECT
    stuid,
    stulname,
    stufname,
    stuaddress
FROM
    uni.student
WHERE
    lower(stulname) LIKE 's%'
    AND lower(stufname) LIKE '%i%'
ORDER BY
    stuid;

-- A4
SELECT
    unitcode,
    ofsemester
FROM
    uni.offering
WHERE
    EXTRACT(YEAR FROM ofyear) = 2021
ORDER BY
    unitcode,
    ofsemester;

-- A5List the year, semester, and unit code for all units
-- that were offered in either semester 2 of 2019 or semester 2 of 2020.
-- Order the output by year and semester then by unit code.
-- To display the offering year correctly in Oracle, you need to use the to_char function.
-- For example, to_char(ofyear,'yyyy').
desc uni.OFFERING;

SELECT
    to_char(ofyear, 'yyyy') AS year,
    ofsemester,
    unitcode
FROM
    uni.offering
WHERE
    (ofsemester = 2
    AND to_char(ofyear, 'yyyy') = '2019')
    OR (ofsemester = 2
    AND to_char(ofyear, 'yyyy') = '2020')
ORDER BY
    year,
    ofsemester,
    unitcode;

-- A6 List the student id, unit code and mark for those students who have failed any unit in semester 2 of 2021.
--  Order the output by student id then order by unit code.
desc uni.ENROLMENT;

SELECT
    stuid,
    unitcode,
    enrolmark
FROM
    uni.enrolment
WHERE
    lower(enrolgrade) = lower('N')
    AND ofsemester = 2
    AND to_char(ofyear, 'yyyy') = '2021'
ORDER BY
    stuid,
    unitcode;

/* Part B - Retrieving data from multiple tables */

-- B1 List all the unit codes, semesters and name of chief examiners
-- (ie. the staff who is responsible for the unit) for all the units that are offered in 2021.
-- Order the output by semester then by unit code.
SELECT
    o.unitcode,
    o.ofsemester,
    s.stafffname
    || ' '
    || s.stafflname AS chief_examiner
FROM
    uni.offering o
    JOIN uni.staff s
    ON o.staffid = s.staffid
WHERE
    to_char(ofyear, 'yyyy') = '2021'
ORDER BY
    o.ofsemester,
    o.unitcode;

SELECT
    *
FROM
    uni.offering
    NATURAL JOIN uni.staff;

SELECT
    *
FROM
    uni.offering
    JOIN uni.staff
    USING (staffid);

-- WARN this implicit join can be inefficient? and is marked down.
SELECT
    *
FROM
    uni.offering o,
    uni.staff    s
WHERE
    o.staffid = s.staffid;

-- B2

SELECT
    e.stuid,
    s.stuname,
    unitcode
FROM
    uni.enrolment e
    JOIN (
        SELECT
            stuid,
            stufname
            || ' '
            || stulname AS stuname
        FROM
            uni.student
    ) s
    ON e.stuid = s.stuid
WHERE
    ofsemester = 1
    AND EXTRACT(YEAR FROM ofyear) = 2021
ORDER BY
    unitcode,
    e.stuid;

-- B3

SELECT
    unitcode,
    ofsemester,
    cltype,
    clday,
    to_char(cltime, 'hh:mm:ss') AS cltime,
    clduration
FROM
    (
        SELECT
            s.unitcode,
            o.ofsemester,
            s.cltype,
            s.clday,
            s.cltime,
            s.clduration,
            s.staffid
        FROM
            uni.offering   o
            JOIN uni.schedclass s
            ON o.unitcode = s.unitcode
    ) os
    JOIN (
        SELECT
            staffid,
            stafffname
            || ' '
            || stafflname AS staffname
        FROM
            uni.staff
        WHERE
            lower(stafffname
                  || ' '
                  || stafflname) = lower('Windham Ellard')
    ) st
    ON os.staffid = st.staffid
ORDER BY
    unitcode,
    ofsemester;

SELECT
    staffid,
    stafffname
    || ' '
    || stafflname AS staffname
FROM
    uni.staff
WHERE
    lower(stafffname
          || ' '
          || stafflname) = lower('Windham Ellard');

-- B4  Create a study statement for Brier Kilgour.
-- A study statement contains unit code, unit name, semester
-- and year the study was attempted, the mark and grade.
-- If the mark and/or grade is unknown, show the mark and/or grade as ‘N/A’.
-- Sort the list by year, then by semester and unit code.

SELECT
    u.unitcode,
    u.unitname,
    ofsemester,
    to_char(ofyear, 'yyyy' )              AS year,
    nvl(to_char(enrolmark, '999'), 'N/A') AS mark,
    nvl(enrolgrade, 'N/A')                AS grade
FROM
    uni.enrolment e
    JOIN (
        SELECT
            stuid
        FROM
            uni.student s
        WHERE
            lower(s.stufname) = lower('Brier')
            AND lower(s.stulname) = lower('Kilgour')
    ) s
    ON e.stuid = s.stuid
    JOIN uni.unit u
    ON u.unitcode = e.unitcode
ORDER BY
    year,
    ofsemester,
    unitcode;

-- B5

SELECT
    u.unitcode  AS unitcode,
    u.unitname  AS unitname,
    p.unitcode  AS prereq_code,
    u2.unitname AS prereq_name
FROM
    uni.unit   u
    JOIN uni.prereq p
    ON u.unitcode = p.unitcode
    JOIN uni.unit u2
    ON u2.unitcode = p.prerequnitcode
ORDER BY
    u.unitcode,
    p.unitcode;

-- B6

SELECT
    p.unitcode  AS prereq_code,
    u2.unitname AS prereq_name
FROM
    (
        SELECT
            u.unitcode,
            u.unitname,
            p.prerequnitcode
        FROM
            uni.unit   u
            JOIN uni.prereq p
            ON u.unitcode = p.unitcode
        WHERE
            lower(u.unitname) = lower('Introduction to data science')
    )        p
    JOIN uni.unit u2
    ON u2.unitcode = p.prerequnitcode
ORDER BY
    p.unitcode;

-- B7

SELECT
    s.stuid,
    s.stufname,
    s.stulname
FROM
    (
        SELECT
            stuid
        FROM
            uni.enrolment
        WHERE
            lower(enrolgrade) = lower('HD')
            AND ofsemester = 2
            AND EXTRACT(YEAR FROM ofyear) = 2021
            AND lower(unitcode) = lower('FIT2094')
    )           e
    JOIN uni.student s
    ON e.stuid = s.stuid
ORDER BY
    s.stuid;

-- Query enrolments for HD

SELECT
    stuid
FROM
    uni.enrolment
WHERE
    lower(enrolgrade) = lower('HD')
    AND ofsemester = 2
    AND EXTRACT(YEAR FROM ofyear) = 2021
    AND lower(unitcode) = lower('FIT2094');

-- B8

SELECT
    s.stufname
    || ' '
    || s.stulname AS stuname,
    e.unitcode
FROM
    (
        SELECT
            stuid,
            unitcode
        FROM
            uni.enrolment
        WHERE
            enrolmark IS NULL
            AND ofsemester = 1
            AND EXTRACT(YEAR FROM ofyear) = 2021
    )           e
    JOIN uni.student s
    ON e.stuid = s.stuid
ORDER BY
    stuname;

-- Query no mark student ids

SELECT
    stuid
FROM
    uni.enrolment
WHERE
    enrolmark IS NULL
    AND ofsemester = 1
    AND EXTRACT(YEAR FROM ofyear) = 2021;

/* Part C - Aggregate Function, Group By and Having */

-- C1

SELECT
    lpad(to_char(AVG(enrolmark), '990.99'), 20) AS average_mark,
    EXTRACT(YEAR FROM ofyear)                   AS year,
    ofsemester
FROM
    uni.enrolment
WHERE
    lower(unitcode) = lower('FIT9136')
GROUP BY
    ofsemester,
    ofyear
ORDER BY
    ofyear,
    ofsemester;

-- C2

SELECT
    COUNT(stuid) AS n_enrolled
FROM
    uni.enrolment
WHERE
    EXTRACT(YEAR FROM ofyear) = 2019
    AND lower(unitcode) = lower('FIT1045');

SELECT
    COUNT(DISTINCT stuid) AS n_enrolled
FROM
    uni.enrolment
WHERE
    EXTRACT(YEAR FROM ofyear) = 2019
    AND lower(unitcode) = lower('FIT1045');

-- C3Find the total number of prerequisite units for each unit which has prerequisites.
-- Order the list by unit code.

SELECT
    p.unitcode,
    COUNT(p.prerequnitcode) AS n_prereqs
FROM
    uni.prereq p
GROUP BY
    p.unitcode
ORDER BY
    p.unitcode;

-- C4

SELECT
    unitcode, COUNT(stuid) AS num_withheld
FROM
    uni.enrolment
WHERE
    lower(enrolgrade) = lower('WH')
GROUP BY
    unitcode
ORDER BY
    num_withheld DESC;

-- C5 Find the total number of enrolments per semester for each unit in the year 2019.
-- The list should include the unitcode, semester and the total number of enrolment.
-- Order the list in increasing order of enrolment numbers.
-- For units with the same number of enrolments,
-- display them by the unitcode order then by the semester order.
SELECT
    e.unitcode,
    e.ofsemester,
    COUNT(e.unitcode) AS enrolments
FROM
    uni.enrolment e
WHERE
    EXTRACT(YEAR FROM ofyear) = 2019
GROUP BY
    e.unitcode,
    e.ofsemester
ORDER BY
    enrolments ASC,
    e.unitcode,
    e.ofsemester;

-- C6


-- C7 Display the unit code and unit name of units
-- which had at least 2 students who were granted a deferred exam (grade is recorded as DEF)
-- in semester 2 2021.
-- Order the list by unit code.
SELECT
    e.unitcode,
    u.unitname,
    e.n_deferred
FROM
    (
        SELECT
            e.unitcode,
            COUNT(e.enrolgrade) AS n_deferred
        FROM
            uni.enrolment e
        WHERE
            upper(e.enrolgrade) = upper('DEF')
            AND e.ofsemester = 2
            AND EXTRACT(YEAR FROM e.ofyear) = 2021
        GROUP BY
            e.unitcode
        HAVING
            COUNT(e.enrolgrade) >= 2
    )        e
    JOIN uni.unit u
    ON u.unitcode = e.unitcode
ORDER BY
    e.unitcode;