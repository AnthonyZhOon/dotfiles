SELECT
    username,
    sid,
    serial#,
    status,
    state,
    to_char(logon_time, 'MONdd hh24:mi') AS "Logon Time"
FROM
    v$session
WHERE
        type = 'USER'
    AND username = user
    AND upper(osuser)!= 'ORACLE'
    AND status != 'KILLED' 
ORDER BY
    "Logon Time";

-- begin
--      kill_own_session(1525,64150);
-- end;