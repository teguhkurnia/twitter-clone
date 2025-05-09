CREATE TABLE post_medias (
  post_id INT NOT NULL REFERENCES posts(id) ON DELETE CASCADE,
  media_id INT NOT NULL REFERENCES medias(id) ON DELETE CASCADE,

  PRIMARY KEY (post_id, media_id)
);