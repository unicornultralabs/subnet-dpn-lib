CREATE TABLE sg_meta (
    id SERIAL NOT NULL PRIMARY KEY,
    c_head BIGINT NOT NULL DEFAULT 0,
    b_from BIGINT NOT NULL DEFAULT 0,
    b_to BIGINT NOT NULL DEFAULT 0
);

INSERT INTO public.sg_meta (id, c_head, b_from, b_to) VALUES (1, 0, 0, 0);
