CREATE TYPE media_type AS ENUM ('image', 'video');
CREATE TYPE moderation_status AS ENUM ('pending', 'approved', 'rejected');

CREATE TABLE medias (
    id SERIAL PRIMARY KEY,
    user_id INT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    media_type media_type NOT NULL DEFAULT 'image',
    mime_type VARCHAR(255) NOT NULL,
    extension VARCHAR(10) NOT NULL,
    width INT NOT NULL,
    height INT NOT NULL,
    duration INT DEFAULT 0,
    size BIGINT NOT NULL,
    path VARCHAR(255) NOT NULL,
    moderation_status moderation_status NOT NULL DEFAULT 'pending',
    moderation_reason VARCHAR(255),
    is_deleted BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP
);

-- Create indexes for faster queries
CREATE INDEX idx_medias_user_id ON medias(user_id);
CREATE INDEX idx_medias_moderation_status ON medias(moderation_status);
CREATE INDEX idx_medias_is_deleted ON medias(is_deleted);
CREATE INDEX idx_medias_media_type ON medias(media_type);