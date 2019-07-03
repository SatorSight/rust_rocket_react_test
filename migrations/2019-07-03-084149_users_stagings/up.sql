CREATE TABLE users_stagings (
    id SERIAL PRIMARY KEY,
    user_id SERIAL references users(id),
    staging_id SERIAL references stagings(id)
)
