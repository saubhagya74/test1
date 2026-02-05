--
-- PostgreSQL database dump
--

\restrict QPVcZDRw1eLdB0L9dDUeMg9RNb7FqqhU5cuvo5caG2elnaqGFkGxSW3XZmO9nrA

-- Dumped from database version 14.20 (Ubuntu 14.20-0ubuntu0.22.04.1)
-- Dumped by pg_dump version 14.20 (Ubuntu 14.20-0ubuntu0.22.04.1)

SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

--
-- Name: compression_label_; Type: TYPE; Schema: public; Owner: devuser
--

CREATE TYPE public.compression_label_ AS ENUM (
    'lz4',
    'gzip',
    'zstd',
    'none'
);


ALTER TYPE public.compression_label_ OWNER TO devuser;

--
-- Name: content_label_; Type: TYPE; Schema: public; Owner: devuser
--

CREATE TYPE public.content_label_ AS ENUM (
    'text',
    'video',
    'audio'
);


ALTER TYPE public.content_label_ OWNER TO devuser;

--
-- Name: encryption_label_; Type: TYPE; Schema: public; Owner: devuser
--

CREATE TYPE public.encryption_label_ AS ENUM (
    'ecc',
    'rsa',
    'none'
);


ALTER TYPE public.encryption_label_ OWNER TO devuser;

SET default_tablespace = '';

SET default_table_access_method = heap;

--
-- Name: conversation_; Type: TABLE; Schema: public; Owner: devuser
--

CREATE TABLE public.conversation_ (
    chat_id_ bigint NOT NULL,
    chat_name_ character varying(30),
    last_message_ character varying(255),
    last_time_ timestamp with time zone DEFAULT CURRENT_TIMESTAMP,
    user_a_id_ bigint NOT NULL,
    user_b_id_ bigint NOT NULL,
    settings_ jsonb DEFAULT '{"theme_": "default", "direction_": [0, 0], "is_pinned_": "false", "notification_level_": "all"}'::jsonb,
    CONSTRAINT check_user_order CHECK ((user_a_id_ < user_b_id_))
);


ALTER TABLE public.conversation_ OWNER TO devuser;

--
-- Name: followed_following_; Type: TABLE; Schema: public; Owner: devuser
--

CREATE TABLE public.followed_following_ (
    follower_id_ bigint NOT NULL,
    following_id_ bigint NOT NULL,
    followed_at timestamp with time zone
);


ALTER TABLE public.followed_following_ OWNER TO devuser;

--
-- Name: group_conversation_; Type: TABLE; Schema: public; Owner: devuser
--

CREATE TABLE public.group_conversation_ (
    group_id_ bigint NOT NULL,
    last_message_ character varying(20),
    last_time_ timestamp with time zone,
    created_at_ timestamp with time zone NOT NULL,
    profile_url_ character varying(300),
    group_settings_ jsonb DEFAULT '{}'::jsonb,
    admin_id_ bigint NOT NULL
);


ALTER TABLE public.group_conversation_ OWNER TO devuser;

--
-- Name: group_member_; Type: TABLE; Schema: public; Owner: devuser
--

CREATE TABLE public.group_member_ (
    group_id_ bigint NOT NULL,
    member_id_ bigint NOT NULL,
    joined_at_ timestamp with time zone
);


ALTER TABLE public.group_member_ OWNER TO devuser;

--
-- Name: group_message_; Type: TABLE; Schema: public; Owner: devuser
--

CREATE TABLE public.group_message_ (
    message_id_ bigint NOT NULL,
    chat_id_ bigint NOT NULL,
    sender_id_ bigint NOT NULL,
    content_type_ public.content_label_,
    description_ character varying(2000),
    messeged_at_ timestamp with time zone NOT NULL,
    compression_type_ public.compression_label_,
    encryption_type_ public.encryption_label_,
    reaction_id_ bigint,
    is_edited_ boolean,
    is_deleted boolean
);


ALTER TABLE public.group_message_ OWNER TO devuser;

--
-- Name: home_; Type: TABLE; Schema: public; Owner: devuser
--

CREATE TABLE public.home_ (
    notification_id_ bigint NOT NULL,
    user_id_ bigint NOT NULL,
    notification_object_ jsonb DEFAULT '{}'::jsonb,
    created_at_ timestamp with time zone NOT NULL,
    is_seen_ boolean DEFAULT false NOT NULL,
    is_deleted_ boolean DEFAULT false NOT NULL
);


ALTER TABLE public.home_ OWNER TO devuser;

--
-- Name: message_; Type: TABLE; Schema: public; Owner: devuser
--

CREATE TABLE public.message_ (
    message_id_ bigint NOT NULL,
    chat_id_ bigint NOT NULL,
    sender_id_ bigint NOT NULL,
    receiver_id_ bigint NOT NULL,
    content_type_ public.content_label_,
    description_ character varying(2000),
    messaged_at_ timestamp with time zone DEFAULT now() NOT NULL,
    compression_type_ public.compression_label_,
    encryption_type_ public.encryption_label_,
    reaction_id_ smallint,
    is_edited_ boolean DEFAULT false,
    is_deleted_ boolean DEFAULT false
);


ALTER TABLE public.message_ OWNER TO devuser;

--
-- Name: reactions_; Type: TABLE; Schema: public; Owner: devuser
--

CREATE TABLE public.reactions_ (
    message_id_ bigint NOT NULL,
    user_id_ bigint NOT NULL,
    emoji_id_ character varying(10) NOT NULL,
    reacted_at_ timestamp with time zone NOT NULL
);


ALTER TABLE public.reactions_ OWNER TO devuser;

--
-- Name: request_; Type: TABLE; Schema: public; Owner: devuser
--

CREATE TABLE public.request_ (
    request_id_ bigint NOT NULL,
    sender_id_ bigint NOT NULL,
    receiver_id_ bigint NOT NULL,
    request_status_ smallint NOT NULL,
    timestamp_ timestamp with time zone NOT NULL
);


ALTER TABLE public.request_ OWNER TO devuser;

--
-- Name: top_reaction_; Type: TABLE; Schema: public; Owner: devuser
--

CREATE TABLE public.top_reaction_ (
    message_id_ bigint NOT NULL,
    top_emoji_ jsonb DEFAULT '{}'::jsonb
);


ALTER TABLE public.top_reaction_ OWNER TO devuser;

--
-- Name: users_; Type: TABLE; Schema: public; Owner: devuser
--

CREATE TABLE public.users_ (
    user_id_ bigint NOT NULL,
    username_ character varying(25) NOT NULL,
    email_ character varying(60),
    password_hash_ text
);


ALTER TABLE public.users_ OWNER TO devuser;

--
-- Name: conversation_ conversation__pkey; Type: CONSTRAINT; Schema: public; Owner: devuser
--

ALTER TABLE ONLY public.conversation_
    ADD CONSTRAINT conversation__pkey PRIMARY KEY (chat_id_);


--
-- Name: followed_following_ followed_following_pkey; Type: CONSTRAINT; Schema: public; Owner: devuser
--

ALTER TABLE ONLY public.followed_following_
    ADD CONSTRAINT followed_following_pkey PRIMARY KEY (follower_id_);


--
-- Name: group_conversation_ group_conversation_pkey; Type: CONSTRAINT; Schema: public; Owner: devuser
--

ALTER TABLE ONLY public.group_conversation_
    ADD CONSTRAINT group_conversation_pkey PRIMARY KEY (group_id_);


--
-- Name: group_member_ group_member__pkey; Type: CONSTRAINT; Schema: public; Owner: devuser
--

ALTER TABLE ONLY public.group_member_
    ADD CONSTRAINT group_member__pkey PRIMARY KEY (group_id_, member_id_);


--
-- Name: group_message_ group_messages__pkey; Type: CONSTRAINT; Schema: public; Owner: devuser
--

ALTER TABLE ONLY public.group_message_
    ADD CONSTRAINT group_messages__pkey PRIMARY KEY (message_id_);


--
-- Name: home_ home__pkey; Type: CONSTRAINT; Schema: public; Owner: devuser
--

ALTER TABLE ONLY public.home_
    ADD CONSTRAINT home__pkey PRIMARY KEY (notification_id_, user_id_);


--
-- Name: message_ messages__pkey; Type: CONSTRAINT; Schema: public; Owner: devuser
--

ALTER TABLE ONLY public.message_
    ADD CONSTRAINT messages__pkey PRIMARY KEY (message_id_);


--
-- Name: reactions_ reactions__pkey; Type: CONSTRAINT; Schema: public; Owner: devuser
--

ALTER TABLE ONLY public.reactions_
    ADD CONSTRAINT reactions__pkey PRIMARY KEY (message_id_);


--
-- Name: request_ request__pkey; Type: CONSTRAINT; Schema: public; Owner: devuser
--

ALTER TABLE ONLY public.request_
    ADD CONSTRAINT request__pkey PRIMARY KEY (request_id_);


--
-- Name: top_reaction_ topreactions__pkey; Type: CONSTRAINT; Schema: public; Owner: devuser
--

ALTER TABLE ONLY public.top_reaction_
    ADD CONSTRAINT topreactions__pkey PRIMARY KEY (message_id_);


--
-- Name: conversation_ unique_user_pair; Type: CONSTRAINT; Schema: public; Owner: devuser
--

ALTER TABLE ONLY public.conversation_
    ADD CONSTRAINT unique_user_pair UNIQUE (user_a_id_, user_b_id_);


--
-- Name: users_ users__pkey; Type: CONSTRAINT; Schema: public; Owner: devuser
--

ALTER TABLE ONLY public.users_
    ADD CONSTRAINT users__pkey PRIMARY KEY (user_id_);


--
-- Name: prevent_duplicate_id; Type: INDEX; Schema: public; Owner: devuser
--

CREATE UNIQUE INDEX prevent_duplicate_id ON public.request_ USING btree (sender_id_, receiver_id_);


--
-- Name: see_unread_notification_; Type: INDEX; Schema: public; Owner: devuser
--

CREATE INDEX see_unread_notification_ ON public.home_ USING btree (user_id_, created_at_ DESC) WHERE ((is_seen_ = false) AND (is_deleted_ = false));


--
-- Name: see_unread_notification_count_; Type: INDEX; Schema: public; Owner: devuser
--

CREATE INDEX see_unread_notification_count_ ON public.home_ USING btree (user_id_) WHERE ((is_seen_ = false) AND (is_deleted_ = false));


--
-- PostgreSQL database dump complete
--

\unrestrict QPVcZDRw1eLdB0L9dDUeMg9RNb7FqqhU5cuvo5caG2elnaqGFkGxSW3XZmO9nrA

