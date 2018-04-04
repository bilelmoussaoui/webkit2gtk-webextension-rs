// This file was generated by gir (https://github.com/gtk-rs/gir @ 6855214)
// from gir-files (https://github.com/gtk-rs/gir-files @ 3fde76b)
// DO NOT EDIT

use ffi;
use glib::translate::*;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum ContextMenuAction {
    NoAction,
    OpenLink,
    OpenLinkInNewWindow,
    DownloadLinkToDisk,
    CopyLinkToClipboard,
    OpenImageInNewWindow,
    DownloadImageToDisk,
    CopyImageToClipboard,
    CopyImageUrlToClipboard,
    OpenFrameInNewWindow,
    GoBack,
    GoForward,
    Stop,
    Reload,
    Copy,
    Cut,
    Paste,
    Delete,
    SelectAll,
    InputMethods,
    Unicode,
    SpellingGuess,
    NoGuessesFound,
    IgnoreSpelling,
    LearnSpelling,
    IgnoreGrammar,
    FontMenu,
    Bold,
    Italic,
    Underline,
    Outline,
    InspectElement,
    OpenVideoInNewWindow,
    OpenAudioInNewWindow,
    CopyVideoLinkToClipboard,
    CopyAudioLinkToClipboard,
    ToggleMediaControls,
    ToggleMediaLoop,
    EnterVideoFullscreen,
    MediaPlay,
    MediaPause,
    MediaMute,
    DownloadVideoToDisk,
    DownloadAudioToDisk,
    Custom,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl ToGlib for ContextMenuAction {
    type GlibType = ffi::WebKitContextMenuAction;

    fn to_glib(&self) -> ffi::WebKitContextMenuAction {
        match *self {
            ContextMenuAction::NoAction => ffi::WEBKIT_CONTEXT_MENU_ACTION_NO_ACTION,
            ContextMenuAction::OpenLink => ffi::WEBKIT_CONTEXT_MENU_ACTION_OPEN_LINK,
            ContextMenuAction::OpenLinkInNewWindow => ffi::WEBKIT_CONTEXT_MENU_ACTION_OPEN_LINK_IN_NEW_WINDOW,
            ContextMenuAction::DownloadLinkToDisk => ffi::WEBKIT_CONTEXT_MENU_ACTION_DOWNLOAD_LINK_TO_DISK,
            ContextMenuAction::CopyLinkToClipboard => ffi::WEBKIT_CONTEXT_MENU_ACTION_COPY_LINK_TO_CLIPBOARD,
            ContextMenuAction::OpenImageInNewWindow => ffi::WEBKIT_CONTEXT_MENU_ACTION_OPEN_IMAGE_IN_NEW_WINDOW,
            ContextMenuAction::DownloadImageToDisk => ffi::WEBKIT_CONTEXT_MENU_ACTION_DOWNLOAD_IMAGE_TO_DISK,
            ContextMenuAction::CopyImageToClipboard => ffi::WEBKIT_CONTEXT_MENU_ACTION_COPY_IMAGE_TO_CLIPBOARD,
            ContextMenuAction::CopyImageUrlToClipboard => ffi::WEBKIT_CONTEXT_MENU_ACTION_COPY_IMAGE_URL_TO_CLIPBOARD,
            ContextMenuAction::OpenFrameInNewWindow => ffi::WEBKIT_CONTEXT_MENU_ACTION_OPEN_FRAME_IN_NEW_WINDOW,
            ContextMenuAction::GoBack => ffi::WEBKIT_CONTEXT_MENU_ACTION_GO_BACK,
            ContextMenuAction::GoForward => ffi::WEBKIT_CONTEXT_MENU_ACTION_GO_FORWARD,
            ContextMenuAction::Stop => ffi::WEBKIT_CONTEXT_MENU_ACTION_STOP,
            ContextMenuAction::Reload => ffi::WEBKIT_CONTEXT_MENU_ACTION_RELOAD,
            ContextMenuAction::Copy => ffi::WEBKIT_CONTEXT_MENU_ACTION_COPY,
            ContextMenuAction::Cut => ffi::WEBKIT_CONTEXT_MENU_ACTION_CUT,
            ContextMenuAction::Paste => ffi::WEBKIT_CONTEXT_MENU_ACTION_PASTE,
            ContextMenuAction::Delete => ffi::WEBKIT_CONTEXT_MENU_ACTION_DELETE,
            ContextMenuAction::SelectAll => ffi::WEBKIT_CONTEXT_MENU_ACTION_SELECT_ALL,
            ContextMenuAction::InputMethods => ffi::WEBKIT_CONTEXT_MENU_ACTION_INPUT_METHODS,
            ContextMenuAction::Unicode => ffi::WEBKIT_CONTEXT_MENU_ACTION_UNICODE,
            ContextMenuAction::SpellingGuess => ffi::WEBKIT_CONTEXT_MENU_ACTION_SPELLING_GUESS,
            ContextMenuAction::NoGuessesFound => ffi::WEBKIT_CONTEXT_MENU_ACTION_NO_GUESSES_FOUND,
            ContextMenuAction::IgnoreSpelling => ffi::WEBKIT_CONTEXT_MENU_ACTION_IGNORE_SPELLING,
            ContextMenuAction::LearnSpelling => ffi::WEBKIT_CONTEXT_MENU_ACTION_LEARN_SPELLING,
            ContextMenuAction::IgnoreGrammar => ffi::WEBKIT_CONTEXT_MENU_ACTION_IGNORE_GRAMMAR,
            ContextMenuAction::FontMenu => ffi::WEBKIT_CONTEXT_MENU_ACTION_FONT_MENU,
            ContextMenuAction::Bold => ffi::WEBKIT_CONTEXT_MENU_ACTION_BOLD,
            ContextMenuAction::Italic => ffi::WEBKIT_CONTEXT_MENU_ACTION_ITALIC,
            ContextMenuAction::Underline => ffi::WEBKIT_CONTEXT_MENU_ACTION_UNDERLINE,
            ContextMenuAction::Outline => ffi::WEBKIT_CONTEXT_MENU_ACTION_OUTLINE,
            ContextMenuAction::InspectElement => ffi::WEBKIT_CONTEXT_MENU_ACTION_INSPECT_ELEMENT,
            ContextMenuAction::OpenVideoInNewWindow => ffi::WEBKIT_CONTEXT_MENU_ACTION_OPEN_VIDEO_IN_NEW_WINDOW,
            ContextMenuAction::OpenAudioInNewWindow => ffi::WEBKIT_CONTEXT_MENU_ACTION_OPEN_AUDIO_IN_NEW_WINDOW,
            ContextMenuAction::CopyVideoLinkToClipboard => ffi::WEBKIT_CONTEXT_MENU_ACTION_COPY_VIDEO_LINK_TO_CLIPBOARD,
            ContextMenuAction::CopyAudioLinkToClipboard => ffi::WEBKIT_CONTEXT_MENU_ACTION_COPY_AUDIO_LINK_TO_CLIPBOARD,
            ContextMenuAction::ToggleMediaControls => ffi::WEBKIT_CONTEXT_MENU_ACTION_TOGGLE_MEDIA_CONTROLS,
            ContextMenuAction::ToggleMediaLoop => ffi::WEBKIT_CONTEXT_MENU_ACTION_TOGGLE_MEDIA_LOOP,
            ContextMenuAction::EnterVideoFullscreen => ffi::WEBKIT_CONTEXT_MENU_ACTION_ENTER_VIDEO_FULLSCREEN,
            ContextMenuAction::MediaPlay => ffi::WEBKIT_CONTEXT_MENU_ACTION_MEDIA_PLAY,
            ContextMenuAction::MediaPause => ffi::WEBKIT_CONTEXT_MENU_ACTION_MEDIA_PAUSE,
            ContextMenuAction::MediaMute => ffi::WEBKIT_CONTEXT_MENU_ACTION_MEDIA_MUTE,
            ContextMenuAction::DownloadVideoToDisk => ffi::WEBKIT_CONTEXT_MENU_ACTION_DOWNLOAD_VIDEO_TO_DISK,
            ContextMenuAction::DownloadAudioToDisk => ffi::WEBKIT_CONTEXT_MENU_ACTION_DOWNLOAD_AUDIO_TO_DISK,
            ContextMenuAction::Custom => ffi::WEBKIT_CONTEXT_MENU_ACTION_CUSTOM,
            ContextMenuAction::__Unknown(value) => value
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::WebKitContextMenuAction> for ContextMenuAction {
    fn from_glib(value: ffi::WebKitContextMenuAction) -> Self {
        skip_assert_initialized!();
        match value {
            0 => ContextMenuAction::NoAction,
            1 => ContextMenuAction::OpenLink,
            2 => ContextMenuAction::OpenLinkInNewWindow,
            3 => ContextMenuAction::DownloadLinkToDisk,
            4 => ContextMenuAction::CopyLinkToClipboard,
            5 => ContextMenuAction::OpenImageInNewWindow,
            6 => ContextMenuAction::DownloadImageToDisk,
            7 => ContextMenuAction::CopyImageToClipboard,
            8 => ContextMenuAction::CopyImageUrlToClipboard,
            9 => ContextMenuAction::OpenFrameInNewWindow,
            10 => ContextMenuAction::GoBack,
            11 => ContextMenuAction::GoForward,
            12 => ContextMenuAction::Stop,
            13 => ContextMenuAction::Reload,
            14 => ContextMenuAction::Copy,
            15 => ContextMenuAction::Cut,
            16 => ContextMenuAction::Paste,
            17 => ContextMenuAction::Delete,
            18 => ContextMenuAction::SelectAll,
            19 => ContextMenuAction::InputMethods,
            20 => ContextMenuAction::Unicode,
            21 => ContextMenuAction::SpellingGuess,
            22 => ContextMenuAction::NoGuessesFound,
            23 => ContextMenuAction::IgnoreSpelling,
            24 => ContextMenuAction::LearnSpelling,
            25 => ContextMenuAction::IgnoreGrammar,
            26 => ContextMenuAction::FontMenu,
            27 => ContextMenuAction::Bold,
            28 => ContextMenuAction::Italic,
            29 => ContextMenuAction::Underline,
            30 => ContextMenuAction::Outline,
            31 => ContextMenuAction::InspectElement,
            32 => ContextMenuAction::OpenVideoInNewWindow,
            33 => ContextMenuAction::OpenAudioInNewWindow,
            34 => ContextMenuAction::CopyVideoLinkToClipboard,
            35 => ContextMenuAction::CopyAudioLinkToClipboard,
            36 => ContextMenuAction::ToggleMediaControls,
            37 => ContextMenuAction::ToggleMediaLoop,
            38 => ContextMenuAction::EnterVideoFullscreen,
            39 => ContextMenuAction::MediaPlay,
            40 => ContextMenuAction::MediaPause,
            41 => ContextMenuAction::MediaMute,
            42 => ContextMenuAction::DownloadVideoToDisk,
            43 => ContextMenuAction::DownloadAudioToDisk,
            10000 => ContextMenuAction::Custom,
            value => ContextMenuAction::__Unknown(value),
        }
    }
}

