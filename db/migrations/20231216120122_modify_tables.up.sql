-- Add up migration script here
DROP TABLE sessions_users;
ALTER TABLE public.transactions ADD CONSTRAINT transactions_fk FOREIGN KEY (user_id) REFERENCES public.users(id) ON DELETE CASCADE;
ALTER TABLE public.user_bandwidth_price RENAME COLUMN bandwidth_price TO rate_per_kb;
ALTER TABLE public.user_bandwidth_price ALTER COLUMN rate_per_kb TYPE int8 USING rate_per_kb::int8;
ALTER TABLE public.user_bandwidth_price ALTER COLUMN rate_per_kb SET DEFAULT 0;
ALTER TABLE public.user_bandwidth_price ADD rate_per_second int8 NOT NULL DEFAULT 0;
ALTER TABLE public.sessions ALTER COLUMN client_id SET NOT NULL;
