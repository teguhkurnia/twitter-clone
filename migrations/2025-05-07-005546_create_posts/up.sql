CREATE TYPE post_type_enum AS ENUM ('post', 'quote', 'repost');

CREATE TABLE posts (
    id SERIAL PRIMARY KEY,
    user_id INT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    parent_id INT REFERENCES posts(id) ON DELETE CASCADE,
    attached_post_id INT REFERENCES posts(id) ON DELETE CASCADE,
    is_thread BOOLEAN NOT NULL DEFAULT FALSE,
    content VARCHAR(5000) NOT NULL,
    is_deleted BOOLEAN NOT NULL DEFAULT FALSE,
    is_edited BOOLEAN NOT NULL DEFAULT FALSE,
    is_pinned BOOLEAN NOT NULL DEFAULT FALSE,
    post_type post_type_enum NOT NULL DEFAULT 'post',
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP
);

-- Create indexes for faster lookups
CREATE INDEX idx_posts_user_id ON posts(user_id);
CREATE INDEX idx_posts_parent_id ON posts(parent_id);
CREATE INDEX idx_posts_attached_post_id ON posts(attached_post_id);
CREATE INDEX idx_posts_is_deleted ON posts(is_deleted);
CREATE INDEX idx_posts_is_edited ON posts(is_edited);
CREATE INDEX idx_posts_is_pinned ON posts(is_pinned);
CREATE INDEX idx_posts_type ON posts(post_type);