-- Your SQL goes here
CREATE TABLE products
(
    id SERIAL PRIMARY KEY,
    title VARCHAR NOT NULL,
    description_short VARCHAR NOT NULL,
    description_long VARCHAR NOT NULL,
    price DOUBLE PRECISION NOT NULL,
    stock INTEGER NOT NULL,
    brand VARCHAR NOT NULL,
    color VARCHAR NOT NULL,
    size VARCHAR NOT NULL
)


    -- id bigint NOT NULL GENERATED ALWAYS AS IDENTITY ( INCREMENT 1 START 1 MINVALUE 1 MAXVALUE 9223372036854775807 CACHE 1 ),
    -- title character varying(100) COLLATE pg_catalog."default" NOT NULL,
    -- description_short character varying(250) COLLATE pg_catalog."default",
    -- description_long character varying(500) COLLATE pg_catalog."default",
    -- price double precision,
    -- stock integer NOT NULL,
    -- brand character varying(75) COLLATE pg_catalog."default",
    -- created_on timestamp without time zone NOT NULL,
    -- color character varying(50) COLLATE pg_catalog."default",
    -- size character varying(100) COLLATE pg_catalog."default",
    -- CONSTRAINT "Product_pkey" PRIMARY KEY (id)
-- )
