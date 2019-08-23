CREATE TABLE users_stagings (
    id SERIAL PRIMARY KEY,
    user_id SERIAL references users(id) on delete cascade,
    staging_id SERIAL references stagings(id) on delete cascade
--     busy boolean default false
)
