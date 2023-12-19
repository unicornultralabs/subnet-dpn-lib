-- Add up migration script here
ALTER TABLE public.transactions ADD created_at bigint NOT NULL DEFAULT 0;
ALTER TABLE public.internal_transactions ADD created_at bigint NOT NULL DEFAULT 0;
