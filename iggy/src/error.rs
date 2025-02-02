use quinn::{ConnectionError, ReadError, ReadToEndError, WriteError};
use std::array::TryFromSliceError;
use std::net::AddrParseError;
use std::num::ParseIntError;
use std::str::Utf8Error;
use thiserror::Error;
use tokio::io;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Empty response")]
    EmptyResponse,
    #[error("Invalid configuration")]
    InvalidConfiguration,
    #[error("Not connected")]
    NotConnected,
    #[error("Request error")]
    RequestError(#[from] reqwest::Error),
    #[error("HTTP response error, status: {0}, body: {1}")]
    HttpResponseError(u16, String),
    #[error("Request middleware error")]
    RequestMiddlewareError(#[from] reqwest_middleware::Error),
    #[error("Cannot create endpoint")]
    CannotCreateEndpoint,
    #[error("Cannot parse URL")]
    CannotParseUrl,
    #[error("Invalid response: {0}")]
    InvalidResponse(u32),
    #[error("Cannot parse address")]
    CannotParseAddress(#[from] AddrParseError),
    #[error("Read error")]
    ReadError(#[from] ReadError),
    #[error("Connection error")]
    ConnectionError(#[from] ConnectionError),
    #[error("Read to end error")]
    ReadToEndError(#[from] ReadToEndError),
    #[error("Error")]
    Error,
    #[error("IO error")]
    IoError(#[from] io::Error),
    #[error("Write error")]
    WriteError(#[from] WriteError),
    #[error("Cannot parse integer")]
    CannotParseInt(#[from] ParseIntError),
    #[error("Cannot parse integer")]
    CannotParseSlice(#[from] TryFromSliceError),
    #[error("Cannot parse UTF8")]
    CannotParseUtf8(#[from] Utf8Error),
    #[error("Invalid command")]
    InvalidCommand,
    #[error("Invalid format")]
    InvalidFormat,
    #[error("Cannot create base directory")]
    CannotCreateBaseDirectory,
    #[error("Cannot create streams directory")]
    CannotCreateStreamsDirectory,
    #[error("Cannot create stream with ID: {0} directory")]
    CannotCreateStreamDirectory(u32),
    #[error("Failed to create stream info file for stream with ID: {0}")]
    CannotCreateStreamInfo(u32),
    #[error("Failed to update stream info for stream with ID: {0}")]
    CannotUpdateStreamInfo(u32),
    #[error("Failed to open stream info file for stream with ID: {0}")]
    CannotOpenStreamInfo(u32),
    #[error("Failed to read stream info file for stream with ID: {0}")]
    CannotReadStreamInfo(u32),
    #[error("Cannot read streams")]
    CannotReadStreams,
    #[error("Failed to create stream with ID: {0}")]
    CannotCreateStream(u32),
    #[error("Failed to delete stream with ID: {0}")]
    CannotDeleteStream(u32),
    #[error("Failed to delete stream directory with ID: {0}")]
    CannotDeleteStreamDirectory(u32),
    #[error("Stream with ID: {0} was not found.")]
    StreamNotFound(u32),
    #[error("Stream with ID: {0} already exists.")]
    StreamAlreadyExists(u32),
    #[error("Invalid stream ID")]
    InvalidStreamId,
    #[error("Invalid stream name")]
    InvalidStreamName,
    #[error("Cannot read topics for stream with ID: {0}")]
    CannotReadTopics(u32),
    #[error("Cannot create topics directory for stream with ID: {0}")]
    CannotCreateTopicsDirectory(u32),
    #[error("Failed to create directory for topic with ID: {0} for stream with ID: {1}.")]
    CannotCreateTopicDirectory(u32, u32),
    #[error("Failed to create topic info file for topic with ID: {0} for stream with ID: {1}.")]
    CannotCreateTopicInfo(u32, u32),
    #[error("Failed to update topic info for topic with ID: {0} for stream with ID: {1}.")]
    CannotUpdateTopicInfo(u32, u32),
    #[error("Failed to open topic info file for topic with ID: {0} for stream with ID: {1}.")]
    CannotOpenTopicInfo(u32, u32),
    #[error("Failed to read topic info file for topic with ID: {0} for stream with ID: {1}.")]
    CannotReadTopicInfo(u32, u32),
    #[error("Failed to create topic with ID: {0} for stream with ID: {1}.")]
    CannotCreateTopic(u32, u32),
    #[error("Failed to delete topic with ID: {0} for stream with ID: {1}.")]
    CannotDeleteTopic(u32, u32),
    #[error("Failed to delete topic directory with ID: {0} for stream with ID: {1}.")]
    CannotDeleteTopicDirectory(u32, u32),
    #[error("Cannot poll topic")]
    CannotPollTopic,
    #[error("Topic with ID: {0} for stream with ID: {1} was not found.")]
    TopicNotFound(u32, u32),
    #[error("Topic with ID: {0} for stream with ID: {1} already exists.")]
    TopicAlreadyExists(u32, u32),
    #[error("Invalid topic ID")]
    InvalidTopicId,
    #[error("Invalid topic name")]
    InvalidTopicName,
    #[error("Invalid topic partitions")]
    InvalidTopicPartitions,
    #[error("Cannot append message")]
    CannotAppendMessage,
    #[error("Cannot create partition with ID: {0} for stream with ID: {1} and topic with ID: {2}")]
    CannotCreatePartition(u32, u32, u32),
    #[error("Failed to create directory for partition with ID: {0} for stream with ID: {1} and topic with ID: {2}")]
    CannotCreatePartitionDirectory(u32, u32, u32),
    #[error(
        "Failed to create directory for partitions for stream with ID: {0} and topic with ID: {1}"
    )]
    CannotCreatePartitionsDirectory(u32, u32),
    #[error(
    "Failed to create directory for consumer groups for stream with ID: {0} and topic with ID: {1}"
    )]
    CannotCreateConsumerGroupsDirectory(u32, u32),
    #[error(
        "Failed to delete partition with ID: {0} for stream with ID: {1} and topic with ID: {2}"
    )]
    CannotDeletePartition(u32, u32, u32),
    #[error("Failed to delete partition directory with ID: {0} for stream with ID: {1} and topic with ID: {2}")]
    CannotDeletePartitionDirectory(u32, u32, u32),
    #[error("Failed to create segment log file for path: {0}.")]
    CannotCreateSegmentLogFile(String),
    #[error("Failed to create segment index file for path: {0}.")]
    CannotCreateSegmentIndexFile(String),
    #[error("Failed to create segment time index file for path: {0}.")]
    CannotCreateSegmentTimeIndexFile(String),
    #[error("Cannot open partition log file")]
    CannotOpenPartitionLogFile,
    #[error(
        "Failed to read partitions directories for topic with ID: {0} and stream with ID: {1}"
    )]
    CannotReadPartitions(u32, u32),
    #[error("Partition with ID: {0} was not found.")]
    PartitionNotFound(u32),
    #[error("Invalid key value length")]
    InvalidKeyValueLength,
    #[error("Invalid messages count")]
    InvalidMessagesCount,
    #[error("Invalid message payload length")]
    InvalidMessagePayloadLength,
    #[error("Segment not found")]
    SegmentNotFound,
    #[error("Segment with start offset: {0} and partition with ID: {1} is closed")]
    SegmentClosed(u64, u32),
    #[error("Segment size is invalid")]
    InvalidSegmentSize(u64),
    #[error("Cannot read message")]
    CannotReadMessage,
    #[error("Cannot read message timestamp")]
    CannotReadMessageTimestamp,
    #[error("Cannot read message ID")]
    CannotReadMessageId,
    #[error("Cannot read message checksum")]
    CannotReadMessageChecksum,
    #[error("Invalid message checksum: {0}, expected: {1}, for offset: {2}")]
    InvalidMessageChecksum(u32, u32, u64),
    #[error("Cannot read message length")]
    CannotReadMessageLength,
    #[error("Cannot read message payload")]
    CannotReadMessagePayload,
    #[error("Cannot save messages to segment")]
    CannotSaveMessagesToSegment,
    #[error("Cannot save index to segment")]
    CannotSaveIndexToSegment,
    #[error("Cannot save time index to segment")]
    CannotSaveTimeIndexToSegment,
    #[error("Empty message payload")]
    EmptyMessagePayload,
    #[error("Too big message payload")]
    TooBigMessagePayload,
    #[error("Too many messages")]
    TooManyMessages,
    #[error("Invalid offset: {0}")]
    InvalidOffset(u64),
    #[error("Failed to read consumers offsets  for partition with ID: {0}")]
    CannotReadConsumerOffsets(u32),
    #[error("Failed to read consumer groups for topic with ID: {0} and stream with ID: {1}")]
    CannotReadConsumerGroups(u32, u32),
    #[error("Failed to create consumer group info file for ID: {0} for topic with ID: {1} for stream with ID: {2}.")]
    CannotCreateConsumerGroupInfo(u32, u32, u32),
    #[error("Failed to delete consumer group info file for ID: {0} for topic with ID: {1} for stream with ID: {2}.")]
    CannotDeleteConsumerGroupInfo(u32, u32, u32),
    #[error("Consumer group with ID: {0} for topic with ID: {1} was not found.")]
    ConsumerGroupNotFound(u32, u32),
    #[error("Consumer group with ID: {0} for topic with ID: {1} already exists.")]
    ConsumerGroupAlreadyExists(u32, u32),
    #[error("Consumer group member with ID: {0} for group with ID: {1} for topic with ID: {2} was not found.")]
    ConsumerGroupMemberNotFound(u32, u32, u32),
    #[error("Invalid consumer group ID")]
    InvalidConsumerGroupId,
    #[error("Feature is unavailable")]
    FeatureUnavailable,
    #[error("Client with ID: {0} was not found.")]
    ClientNotFound(u32),
    #[error("Invalid client ID")]
    InvalidClientId,
}

impl Error {
    pub fn as_code(&self) -> u32 {
        match self {
            Error::Error => 1,
            Error::InvalidConfiguration => 2,
            Error::InvalidCommand => 3,
            Error::InvalidFormat => 4,
            Error::FeatureUnavailable => 5,
            Error::CannotCreateBaseDirectory => 10,
            Error::NotConnected => 51,
            Error::RequestError(_) => 52,
            Error::ClientNotFound(_) => 100,
            Error::InvalidClientId => 101,
            Error::IoError(_) => 200,
            Error::WriteError(_) => 201,
            Error::CannotParseUtf8(_) => 202,
            Error::CannotParseInt(_) => 203,
            Error::CannotParseSlice(_) => 204,
            Error::HttpResponseError(_, _) => 300,
            Error::RequestMiddlewareError(_) => 301,
            Error::CannotCreateEndpoint => 302,
            Error::CannotParseUrl => 303,
            Error::InvalidResponse(_) => 304,
            Error::EmptyResponse => 305,
            Error::CannotParseAddress(_) => 306,
            Error::ReadError(_) => 307,
            Error::ConnectionError(_) => 308,
            Error::ReadToEndError(_) => 309,
            Error::CannotCreateStreamsDirectory => 1000,
            Error::CannotCreateStreamDirectory(_) => 1001,
            Error::CannotCreateStreamInfo(_) => 1002,
            Error::CannotUpdateStreamInfo(_) => 1003,
            Error::CannotOpenStreamInfo(_) => 1004,
            Error::CannotReadStreamInfo(_) => 1005,
            Error::CannotCreateStream(_) => 1006,
            Error::CannotDeleteStream(_) => 1007,
            Error::CannotDeleteStreamDirectory(_) => 1008,
            Error::StreamNotFound(_) => 1009,
            Error::StreamAlreadyExists(_) => 1010,
            Error::InvalidStreamName => 1011,
            Error::InvalidStreamId => 1012,
            Error::CannotReadStreams => 1013,
            Error::CannotCreateTopicsDirectory(_) => 2000,
            Error::CannotCreateTopicDirectory(_, _) => 2001,
            Error::CannotCreateTopicInfo(_, _) => 2002,
            Error::CannotUpdateTopicInfo(_, _) => 2003,
            Error::CannotOpenTopicInfo(_, _) => 2004,
            Error::CannotReadTopicInfo(_, _) => 2005,
            Error::CannotCreateTopic(_, _) => 2006,
            Error::CannotDeleteTopic(_, _) => 2007,
            Error::CannotDeleteTopicDirectory(_, _) => 2008,
            Error::CannotPollTopic => 2009,
            Error::TopicNotFound(_, _) => 2010,
            Error::TopicAlreadyExists(_, _) => 2011,
            Error::InvalidTopicName => 2012,
            Error::InvalidTopicPartitions => 2013,
            Error::InvalidTopicId => 2014,
            Error::CannotReadTopics(_) => 2015,
            Error::CannotCreatePartition(_, _, _) => 3000,
            Error::CannotCreatePartitionsDirectory(_, _) => 3001,
            Error::CannotCreatePartitionDirectory(_, _, _) => 3002,
            Error::CannotOpenPartitionLogFile => 3003,
            Error::CannotReadPartitions(_, _) => 3004,
            Error::CannotDeletePartition(_, _, _) => 3005,
            Error::CannotDeletePartitionDirectory(_, _, _) => 3006,
            Error::PartitionNotFound(_) => 3007,
            Error::SegmentNotFound => 4000,
            Error::SegmentClosed(_, _) => 4001,
            Error::InvalidSegmentSize(_) => 4002,
            Error::CannotCreateSegmentLogFile(_) => 4003,
            Error::CannotCreateSegmentIndexFile(_) => 4004,
            Error::CannotCreateSegmentTimeIndexFile(_) => 4005,
            Error::CannotSaveMessagesToSegment => 4006,
            Error::CannotSaveIndexToSegment => 4007,
            Error::CannotSaveTimeIndexToSegment => 4008,
            Error::InvalidMessagesCount => 4009,
            Error::CannotAppendMessage => 4010,
            Error::CannotReadMessage => 4011,
            Error::CannotReadMessageTimestamp => 4012,
            Error::CannotReadMessageId => 4013,
            Error::CannotReadMessageLength => 4014,
            Error::CannotReadMessagePayload => 4015,
            Error::TooBigMessagePayload => 4016,
            Error::TooManyMessages => 4017,
            Error::EmptyMessagePayload => 4018,
            Error::InvalidMessagePayloadLength => 4019,
            Error::CannotReadMessageChecksum => 4020,
            Error::InvalidMessageChecksum(_, _, _) => 4021,
            Error::InvalidKeyValueLength => 4022,
            Error::InvalidOffset(_) => 4100,
            Error::CannotReadConsumerOffsets(_) => 4101,
            Error::ConsumerGroupNotFound(_, _) => 5000,
            Error::ConsumerGroupAlreadyExists(_, _) => 5001,
            Error::ConsumerGroupMemberNotFound(_, _, _) => 5002,
            Error::InvalidConsumerGroupId => 5003,
            Error::CannotCreateConsumerGroupsDirectory(_, _) => 5004,
            Error::CannotReadConsumerGroups(_, _) => 5005,
            Error::CannotCreateConsumerGroupInfo(_, _, _) => 5006,
            Error::CannotDeleteConsumerGroupInfo(_, _, _) => 5007,
        }
    }

    pub fn from_code_as_string(code: u32) -> &'static str {
        match code {
            1 => "error",
            2 => "invalid_configuration",
            3 => "invalid_command",
            4 => "invalid_format",
            5 => "feature_unavailable",
            10 => "cannot_create_base_directory",
            51 => "not_connected",
            52 => "request_error",
            100 => "client_not_found",
            101 => "invalid_client_id",
            200 => "io_error",
            201 => "write_error",
            202 => "cannot_parse_utf8",
            203 => "cannot_parse_int",
            204 => "cannot_parse_slice",
            300 => "http_response_error",
            301 => "request_middleware_error",
            302 => "cannot_create_endpoint",
            303 => "cannot_parse_url",
            304 => "invalid_response",
            305 => "empty_response",
            306 => "cannot_parse_address",
            307 => "read_error",
            308 => "connection_error",
            309 => "read_to_end_error",
            1000 => "cannot_create_streams_directory",
            1001 => "cannot_create_stream_directory",
            1002 => "cannot_create_stream_info",
            1003 => "cannot_update_stream_info",
            1004 => "cannot_open_stream_info",
            1005 => "cannot_read_stream_info",
            1006 => "cannot_create_stream",
            1007 => "cannot_delete_stream",
            1008 => "cannot_delete_stream_directory",
            1009 => "stream_not_found",
            1010 => "stream_already_exists",
            1011 => "invalid_stream_name",
            1012 => "invalid_stream_id",
            1013 => "cannot_read_streams",
            2000 => "cannot_create_topics_directory",
            2001 => "cannot_create_topic_directory",
            2002 => "cannot_create_topic_info",
            2003 => "cannot_update_topic_info",
            2004 => "cannot_open_topic_info",
            2005 => "cannot_read_topic_info",
            2006 => "cannot_create_topic",
            2007 => "cannot_delete_topic",
            2008 => "cannot_delete_topic_directory",
            2009 => "cannot_poll_topic",
            2010 => "topic_not_found",
            2011 => "topic_already_exists",
            2012 => "invalid_topic_name",
            2013 => "invalid_topic_partitions",
            2014 => "invalid_topic_id",
            2015 => "cannot_read_topics",
            3000 => "cannot_create_partition",
            3001 => "cannot_create_partitions_directory",
            3002 => "cannot_create_partition_directory",
            3003 => "cannot_open_partition_log_file",
            3004 => "cannot_read_partitions",
            3005 => "cannot_delete_partition",
            3006 => "cannot_delete_partition_directory",
            3007 => "partition_not_found",
            4000 => "segment_not_found",
            4001 => "segment_closed",
            4002 => "invalid_segment_size",
            4003 => "cannot_create_segment_log_file",
            4004 => "cannot_create_segment_index_file",
            4005 => "cannot_create_segment_time_index_file",
            4006 => "cannot_save_messages_to_segment",
            4007 => "cannot_save_index_to_segment",
            4008 => "cannot_save_time_index_to_segment",
            4009 => "invalid_messages_count",
            4010 => "cannot_append_message",
            4011 => "cannot_read_message",
            4012 => "cannot_read_message_timestamp",
            4013 => "cannot_read_message_id",
            4014 => "cannot_read_message_length",
            4015 => "cannot_read_message_payload",
            4016 => "too_big_message_payload",
            4017 => "too_many_messages",
            4018 => "empty_message_payload",
            4019 => "invalid_message_payload_length",
            4020 => "cannot_read_message_checksum",
            4021 => "invalid_message_checksum",
            4022 => "invalid_key_value_length",
            4100 => "invalid_offset",
            4101 => "cannot_read_consumer_offsets",
            5000 => "consumer_group_not_found",
            5001 => "consumer_group_already_exists",
            5002 => "consumer_group_member_not_found",
            5003 => "invalid_consumer_group_id",
            5004 => "cannot_create_consumer_groups_directory",
            5005 => "cannot_read_consumer_groups",
            5006 => "cannot_create_consumer_group_info",
            5007 => "cannot_delete_consumer_group_info",
            _ => "error",
        }
    }

    pub fn as_string(&self) -> &'static str {
        match self {
            Error::Error => "error",
            Error::IoError(_) => "io_error",
            Error::InvalidCommand => "invalid_command",
            Error::InvalidFormat => "invalid_format",
            Error::CannotCreateBaseDirectory => "cannot_create_base_directory",
            Error::CannotCreateStreamsDirectory => "cannot_create_streams_directory",
            Error::CannotCreateStreamDirectory(_) => "cannot_create_stream_directory",
            Error::CannotCreateStreamInfo(_) => "cannot_create_stream_info",
            Error::CannotUpdateStreamInfo(_) => "cannot_update_stream_info",
            Error::CannotOpenStreamInfo(_) => "cannot_open_stream_info",
            Error::CannotReadStreamInfo(_) => "cannot_read_stream_info",
            Error::CannotCreateStream(_) => "cannot_create_stream",
            Error::CannotDeleteStream(_) => "cannot_delete_stream",
            Error::CannotDeleteStreamDirectory(_) => "cannot_delete_stream_directory",
            Error::StreamNotFound(_) => "stream_not_found",
            Error::StreamAlreadyExists(_) => "stream_already_exists",
            Error::InvalidStreamName => "invalid_stream_name",
            Error::CannotCreateTopicsDirectory(_) => "cannot_create_topics_directory",
            Error::CannotCreateTopicDirectory(_, _) => "cannot_create_topic_directory",
            Error::CannotCreateTopicInfo(_, _) => "cannot_create_topic_info",
            Error::CannotUpdateTopicInfo(_, _) => "cannot_update_topic_info",
            Error::CannotOpenTopicInfo(_, _) => "cannot_open_topic_info",
            Error::CannotReadTopicInfo(_, _) => "cannot_read_topic_info",
            Error::CannotCreateTopic(_, _) => "cannot_create_topic",
            Error::CannotDeleteTopic(_, _) => "cannot_delete_topic",
            Error::CannotDeleteTopicDirectory(_, _) => "cannot_delete_topic_directory",
            Error::CannotPollTopic => "cannot_poll_topic",
            Error::TopicNotFound(_, _) => "topic_not_found",
            Error::TopicAlreadyExists(_, _) => "topic_already_exists",
            Error::InvalidTopicName => "invalid_topic_name",
            Error::InvalidTopicPartitions => "invalid_topic_partitions",
            Error::CannotAppendMessage => "cannot_append_message",
            Error::CannotCreatePartition(_, _, _) => "cannot_create_partition",
            Error::CannotCreatePartitionDirectory(_, _, _) => "cannot_create_partition_directory",
            Error::CannotCreateSegmentLogFile(_) => "cannot_create_segment_log_file",
            Error::CannotCreateSegmentIndexFile(_) => "cannot_create_segment_index_file",
            Error::CannotCreateSegmentTimeIndexFile(_) => "cannot_create_segment_time_index_file",
            Error::CannotOpenPartitionLogFile => "cannot_open_partition_log_file",
            Error::CannotReadPartitions(_, _) => "cannot_read_partitions",
            Error::PartitionNotFound(_) => "partition_not_found",
            Error::InvalidMessagesCount => "invalid_messages_count",
            Error::InvalidStreamId => "invalid_stream_id",
            Error::InvalidTopicId => "invalid_topic_id",
            Error::SegmentNotFound => "segment_not_found",
            Error::SegmentClosed(_, _) => "segment_closed",
            Error::InvalidSegmentSize(_) => "invalid_segment_size",
            Error::CannotReadMessage => "cannot_read_message",
            Error::CannotReadMessageTimestamp => "cannot_read_message_timestamp",
            Error::CannotReadMessageId => "cannot_read_message_id",
            Error::CannotReadMessageLength => "cannot_read_message_length",
            Error::CannotReadMessagePayload => "cannot_read_message_payload",
            Error::CannotSaveMessagesToSegment => "cannot_save_messages_to_segment",
            Error::CannotSaveIndexToSegment => "cannot_save_index_to_segment",
            Error::CannotSaveTimeIndexToSegment => "cannot_save_time_index_to_segment",
            Error::CannotParseUtf8(_) => "cannot_parse_utf8",
            Error::CannotParseInt(_) => "cannot_parse_int",
            Error::CannotParseSlice(_) => "cannot_parse_slice",
            Error::TooBigMessagePayload => "too_big_message_payload",
            Error::TooManyMessages => "too_many_messages",
            Error::WriteError(_) => "write_error",
            Error::InvalidOffset(_) => "invalid_offset",
            Error::CannotReadConsumerOffsets(_) => "cannot_read_consumer_offsets",
            Error::CannotDeletePartition(_, _, _) => "cannot_delete_partition",
            Error::CannotDeletePartitionDirectory(_, _, _) => "cannot_delete_partition_directory",
            Error::InvalidMessagePayloadLength => "invalid_message_payload_length",
            Error::EmptyMessagePayload => "empty_message_payload",
            Error::CannotReadStreams => "cannot_read_streams",
            Error::CannotReadTopics(_) => "cannot_read_topics",
            Error::CannotReadMessageChecksum => "cannot_read_message_checksum",
            Error::InvalidMessageChecksum(_, _, _) => "invalid_message_checksum",
            Error::InvalidKeyValueLength => "invalid_key_value_length",
            Error::ConsumerGroupNotFound(_, _) => "consumer_group_not_found",
            Error::ConsumerGroupAlreadyExists(_, _) => "consumer_group_already_exists",
            Error::ConsumerGroupMemberNotFound(_, _, _) => "consumer_group_member_not_found",
            Error::FeatureUnavailable => "feature_unavailable",
            Error::CannotCreatePartitionsDirectory(_, _) => "cannot_create_partitions_directory",
            Error::CannotCreateConsumerGroupsDirectory(_, _) => {
                "cannot_create_consumer_groups_directory"
            }
            Error::CannotReadConsumerGroups(_, _) => "cannot_read_consumer_groups",
            Error::CannotCreateConsumerGroupInfo(_, _, _) => "cannot_create_consumer_group_info",
            Error::CannotDeleteConsumerGroupInfo(_, _, _) => "cannot_delete_consumer_group_info",
            Error::ClientNotFound(_) => "client_not_found",
            Error::InvalidClientId => "invalid_client_id",
            Error::EmptyResponse => "empty_response",
            Error::InvalidConfiguration => "invalid_configuration",
            Error::NotConnected => "not_connected",
            Error::RequestError(_) => "request_error",
            Error::HttpResponseError(_, _) => "http_response_error",
            Error::RequestMiddlewareError(_) => "request_middleware_error",
            Error::CannotCreateEndpoint => "cannot_create_endpoint",
            Error::CannotParseUrl => "cannot_parse_url",
            Error::InvalidResponse(_) => "invalid_response",
            Error::CannotParseAddress(_) => "cannot_parse_address",
            Error::ReadError(_) => "read_error",
            Error::ConnectionError(_) => "connection_error",
            Error::ReadToEndError(_) => "read_to_end_error",
            Error::InvalidConsumerGroupId => "invalid_consumer_group_id",
        }
    }
}
