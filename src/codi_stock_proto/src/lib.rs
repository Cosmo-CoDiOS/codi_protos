#![deny(
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    clippy::all,
    clippy::pedantic,
    clippy::cargo,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unused_import_braces,
    unused_qualifications,
    unused_extern_crates,
    variant_size_differences
)]
#![no_std]

//! This module specifies the protocol uses by the Mediatek chip and `CoDi` chip.

// Protocol docs to be added later - `non_camel_case_types` lint must remain allowed.
#[allow(missing_docs, non_camel_case_types, dead_code)]
#[derive(Debug, Copy, Clone)]
/// This holds an enum of the protocol used to communicate with the Cover Display.
pub enum StockCoDiProtocol {
    CMD_MTK_GET_PROTOCOL_VERSION = 0,
    CMD_MTK_GET_CODI_FLASH_VERSION = 1,
    CMD_ST32_INFO_CODI_FLASH_VERSION = 2,
    CMD_ST32_INFO_PROTOCOL_VERSION = 3,
    CMD_ST32_SET_BINARY = 4,
    CMD_ST32_SET_S8 = 5,
    CMD_ST32_RESTART = 6,
    CMD_ST32_GET_DATETIME = 7,
    CMD_MTK_INFO_DATETIME = 8,
    CMD_ST32_SET_LOCATION_STATUS = 9,
    CMD_ST32_GET_LOCATION_STATUS = 10,
    CMD_MTK_INFO_LOCATION_STATUS = 11,
    CMD_ST32_SET_TORCH_STATUS = 12,
    CMD_ST32_GET_TORCH_STATUS = 13,
    CMD_MTK_INFO_TORCH_STATUS = 14,
    CMD_ST32_GET_COVER_STATUS = 15,
    CMD_MTK_INFO_COVER_STATUS = 16,
    CMD_ST32_SET_WIFI_STATUS = 17,
    CMD_ST32_GET_WIFI_STATUS = 18,
    CMD_MTK_INFO_WIFI_STATUS = 19,
    CMD_ST32_SET_BT_STATUS = 20,
    CMD_ST32_GET_BT_STATUS = 21,
    CMD_MTK_INFO_BT_STATUS = 22,
    CMD_ST32_SET_BATTERY_SAVER_STATUS = 23,
    CMD_ST32_GET_BATTERY_SAVER_STATUS = 24,
    CMD_MTK_INFO_BATTERY_SAVER_STATUS = 25,
    CMD_ST32_SET_FLIGHT_MODE_STATUS = 26,
    CMD_ST32_GET_FLIGHT_MODE_STATUS = 27,
    CMD_MTK_INFO_FLIGHT_MODE_STATUS = 28,
    CMD_ST32_SET_HOTSPOT_STATUS = 29,
    CMD_ST32_GET_HOTSPOT_STATUS = 30,
    CMD_MTK_INFO_HOTSPOT_STATUS = 31,
    CMD_ST32_SET_MOBILE_DATA_STATUS = 32,
    CMD_ST32_GET_MOBILE_DATA_STATUS = 33,
    CMD_MTK_INFO_MOBILE_DATA_STATUS = 34,
    CMD_ST32_SET_DND_STATUS = 35,
    CMD_ST32_GET_DND_STATUS = 36,
    CMD_MTK_INFO_DND_STATUS = 37,
    CMD_ST32_SET_VOLUME_LEVEL = 38,
    CMD_ST32_GET_VOLUME_LEVEL = 39,
    CMD_MTK_INFO_VOLUME_LEVEL = 40,
    CMD_ST32_GET_BATTERY_LEVEL = 41,
    CMD_MTK_INFO_BATTERY_LEVEL = 42,
    CMD_ST32_INFO_CODI_STATUS = 43,
    CMD_MTK_SET_CODI_STATUS = 121,
    CMD_ST32_SET_LOCK = 45,
    CMD_ST32_GET_LOCK_STATUS = 46,
    CMD_MTK_INFO_LOCK_STATUS = 47,
    CMD_ST32_DISMISS_CALL_SMS = 48,
    CMD_ST32_ACTION_UNLOCK = 49,
    CMD_ST32_INFO_ST_CHARGING = 50,
    CMD_ST32_PLAY_DTMF = 51,
    CMD_ST32_SEND_DTMF = 52,
    CMD_ST32_ACTION_CALL = 53,
    CMD_ST32_SEND_TELE_CODE = 54,
    CMD_ST32_SET_CALL_MUTE_STATUS = 55,
    CMD_ST32_GET_CALL_MUTE_STATUS = 56,
    CMD_MTK_INFO_CALL_MUTE_STATUS = 57,
    CMD_ST32_SET_CALL_OUTPUT = 58,
    CMD_ST32_GET_CALL_OUTPUT = 59,
    CMD_MTK_INFO_CALL_OUTPUT = 60,
    CMD_ST32_GET_CALL_OUTPUT_OPTIONS = 61,
    CMD_MTK_INFO_CALL_OUTPUT_OPTIONS = 62,
    CMD_ST32_ACTION_CAMERA = 63,
    CMD_ST32_GET_CAMERA_FRAME = 64,
    CMD_ST32_SET_CAMERA_SETTINGS = 65,
    CMD_ST32_CAMERA_CAPTURE_IMAGE = 66,
    CMD_ST32_ACTION_VIDEO = 67,
    CMD_ST32_GET_VIDEO_FRAME = 68,
    CMD_ST32_SET_VIDEO_SETTINGS = 69,
    CMD_ST32_VIDEO_CAPTURE_IMAGE = 70,
    CMD_MTK_INFO_CAMERA_STATUS = 71,
    CMD_MTK_INFO_CAMERA_SETTINGS = 72,
    CMD_MTK_INFO_VIDEO_STATUS = 73,
    CMD_MTK_INFO_VIDEO_SETTINGS = 74,
    CMD_MTK_INFO_COVER_LIGHT_SENSOR = 75,
    CMD_ST32_GET_COVER_LIGHT_SENSOR = 76,
    CMD_MTK_LOAD_LANGUAGE_RESOURCE = 77,
    CMD_MTK_GET_CURRENT_LANGUAGE = 78,
    CMD_ST32_INFO_CURRENT_LANGUAGE = 79,
    CMD_MTK_SET_CURRENT_LANGUAGE = 80,
    CMD_MTK_SHOW_MEDIA = 81,
    CMD_MTK_STOP_MEDIA = 82,
    CMD_MTK_LOAD_MEDIA = 83,
    CMD_MTK_UNLOAD_MEDIA = 84,
    CMD_MTK_HAS_MEDIA = 85,
    CMD_ST32_INFO_MEDIA_RESOURCE = 86,
    CMD_ST32_INFO_MEDIA_ACTIVITY = 87,
    CMD_MTK_SHOW_ALERT = 88,
    CMD_ST32_INFO_ALERT = 89,
    CMD_MTK_STOP_ALERT = 90,
    CMD_MTK_ORIENTATION_INFO = 91,
    CMD_ST32_GET_ORIENTATION = 92,
    CMD_MTK_ACTION_CODI_HOME = 93,
    CMD_MTK_INFO_NEXT_ALARM = 94,
    CMD_MTK_SHOW_BATTERY_LEVEL = 95,
    CMD_ST32_GET_CALL_HISTORY = 96,
    CMD_ST32_GET_CONTACTS = 97,
    CMD_MTK_CONTACT_INFO = 98,
    CMD_MTK_CALL_HISTORY_INFO = 99,
    CMD_MTK_NOTIFICATION_INFO = 100,
    CMD_MTK_PLAYER_INFO = 101,
    CMD_ST32_ACTION_PLAYER = 102,
    CMD_MTK_CALL_INFO = 103,
    CMD_ST32_ACTION_NOTIFICATION = 104,
    CMD_ST32_GET_LEDISON_PATTERN = 105,
    CMD_MTK_LEDISON_MODE_INFO = 106,
    CMD_ST32_GET_LEDISON_MODE = 107,
    CMD_MTK_LEDISON_PATTERN_INFO = 108,
    CMD_ST32_GET_CONTACT_ICON = 109,
    CMD_MTK_CONTACT_ICON_INFO = 110,
    CMD_MTK_MODEM_SIGNAL_INFO = 111,
    CMD_MTK_WEATHER_INFO = 112,
    CMD_MTK_EXTRA_COMMAND = 113,
    CMD_ST32_GET_MODEM_SIGNAL_INFO = 114,
    CMD_ST32_GET_DATE_TIME_FORMAT = 115,
    CMD_MTK_DATE_TIME_FORMAT_INFO = 116,
    CMD_ST32_GET_ALBUM_ART = 117,
    CMD_MTK_ALBUM_ART_INFO = 118,
    CMD_MTK_CAMERA_FRAME_IMG = 119,
    CMD_MTK_KEY_PRESS_INFO = 120,
    CMD_ST32_ACTION_VOICE_RECODER = 122,
    CMD_ST32_SET_VOICE_RECORDER_SETTINGS = 123,
    CMD_MTK_INFO_VOICE_RECODER_SETTINGS = 124,
    CMD_MTK_INFO_VOICE_RECORDER_STATUS = 125,
    CMD_MTK_DATA_CHANGE_ALERT = 126,
    CMD_ST32_DATA_CHANGE_ALERT = 127,
    CMD_AEON_MTK_SET_ST32_RESET = 140,
    CMD_GET_ST32_SW_VERSION = 141,
    CMD_SYNC_USB_STATUS = 142,
    CMD_SYNC_SYS_SLEEP_STATUS = 143,
    CMD_SYNC_RIGHT_USB_OTG_STATUS = 144,
    CMD_ST_ENTRY_DEEP_SLEEP_STATUS = 145,
}
