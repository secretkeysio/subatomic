;(function(styles) {
    document.addEventListener('DOMContentLoaded', function() {
        var style = document.createElement('style');
        style.innerHTML = styles.join('');
        (document.head ? document.head : document.body).appendChild(style);
    }, false);
})([
    // Login screen
    '::-webkit-input-placeholder { font-style: normal !important; }',
    'h1.aligncenter, .headerNoAuth-container, .pm_panel-bottom, .loginForm-actions-help, .loginForm-actions-new-title, .loginForm-actions-create-container { display: none !important; }',
    '.pm_panel-login__inner { background: transparent; border: 0 !important; border-radius: 0 !important; padding-top: 85px !important; }',
    '.pm_panel-login__inner input[type=text], .pm_panel-login__inner .pm-field-icon-container { border-radius: 16px !important; }',
    '.pm_panel-login__inner input[type=text] { padding-top: 7px; padding-bottom: 7px; }',
    '.pm_panel-login__inner input[type=password] { background: transparent !important; }',
    'html body .pm_panel-login__inner input.password-input { border: 0 !important; }',
    '#login_btn { border-radius: 16px !important; padding: 7px 0 !important; }',
    '#pm_footer { font-size: 12px !important; }',

    // Mailbox
    '#aside-bar { display: none; }',
    '.main { border-radius: 0 !important; margin-right: 0 !important; }',
    '.content { background-image: none !important; }',
    '.toolbar { box-shadow: none !important; border-radius: 0 !important; }',
    '.main-area, .main-area--noHeader, .main-area--withToolbar, .main-area--withToolbar--noHeader { border-radius: 0 !important; }',
    '.item-container { padding: 10px !important; }',
    '.item-columns-list { background: none !important; }',
    '.toolbar-separator { width: 1px !important; }',
    '.container-section-sticky { padding-bottom: 2em !important; }',
    '.message-container { border-radius: 2px !important; }',
    '.pm-group-buttons>.pm-group-button:first-child, .pm-group-buttons>.pm-group-button:last-child { border-radius: 0 !important; }',

    // We hide this as we do notifications on our side - the nav item is trickier to hide, sadly
    'section[data-target-id=desktop-notifications] { display: none; }',
    '.message-container:not(.sent):not(.draft) .message-header.message-summary:before, .message-header.is-inbound:before, .message-header.is-outbound:before { content: none !important; }',

    '.subject { font-weight: bold !important; font-size: 1.3rem !important; }',
    '.meta, .senders { font-size: 1.3rem !important; }',

    '@media (prefers-color-scheme: dark) {',
        '#login { background-color: #323232 !important; }',
        'html, body { background: #323232 !important; }',
        '.link:active, .link:focus, .link:hover, a:active, a:focus, a:hover { color: #788ee8; }',
        '.pm-modal { background: #181818 !important; color: #ddd !important; }',
        '.pm-modalContentInner { background: #181818 !important; background-image: none !important; }',
        '.pm-modalContentInner:after, .pm-modalContentInner:before { background: #181818 !important; }',
        '.toolbar { background: #222 !important; border-bottom: 1px solid #171717; border-top: 1px solid #1e1e1e !important; border-left: 1px solid #1e1e1e !important; border-top-left-radius: 4px !important; }',
        '#wrapper { border-left: 1px solid #171717 !important; }',
        '.items-column-list, .view-column-detail { background: #1E1E1E !important; }',
        '.items-column-list { border-right-color: #171717 !important; }',
        '.item-icon { border-color: #2a2a2a !important; background: #2b2b2b !important; }',
        '.item-checkbox:checked+.item-icon { background-color: #657ee4 !important; border-color: #4c69e0 !important; }',
        '.conversation.marked:before { background: transparent !important; }',
        '.item-wrapper { background: #1E1E1E !important; }',
        '.conversation { border-left: 2px solid #1e1e1e; }',
        '.item-container:not(.item-is-selected):not(.read):not(.active), .conversation:not(.active):not(.read) { border-left: 2px solid #307ace; background: #1b1b1b !important; }',
        '.item-container, .item-container-row { border-bottom-color: #343434 !important; }',
    'html .item-container.active, html .item-is-selected { background: #333 !important; border-left-color: #333 !important; }',
        '.toolbar-separator { background: #343434 !important; }',
        '.item-container.read { border-left-color: #1E1E1E; }',
        '.pm-field, .pm-field-icon-container { color: #ddd !important; }',
        '.main-area, .main-area--noHeader, .main-area--withToolbar, .main-area--withToolbar--noHeader, .main-area-content { color: #ddd; background: #1E1E1E !important; }',
        '.bg-global-light { background: #1E1E1E !important; }',
        '.dropDown-content { background: #2a2a2a !important; color: #ddd !important; }',
        '.dropDown-contentInner:after, .dropDown-contentInner:before { background: #2a2a2a !important; }',
        '.dropDown-item+.dropDown-item { border-top-color: #555 !important; }',
        '.dropDown--bottom-left:before, .dropDown--bottom-right:before, .dropDown--bottom:before, .dropDown-content:before { border-bottom-color: #2a2a2a; }',
        '.dropDown-contentInner { background-image: none !important; background: #2a2a2a !important; }',
        '[class*=block-info] { background: transparent !important; color: #788ee8 !important; }',
        '.meta, .senders { color: #777 !important; }',

        '[aria-current=page].navigation__link { background: #393939 !important; }',
        '.meta, .senders { color: #777 !important; }',
        '.dropDown-content--advancedSearch .pm-field, #pm_composer .pm-field { border: 1px solid #181818 !important; background: #777 !important; }',
        '.dropDown-content--advancedSearch .focus.pm-field-icon-container, .dropDown-content--advancedSearch .pm-field-icon-container:focus, .dropDown-content--advancedSearch .pm-field-icon-container:focus-within, .dropDown-content--advancedSearch .pm-field.focus, .dropDown-content--advancedSearch .pm-field:focus, .dropDown-content--advancedSearch .pm-field:focus-within { background: #555 !important; }',

        '#conversation-view { padding: 16px !important; }',
        '.message-container { border-color: #1b1b1b; border-radius: 2px !important; }',
        '.message-header { background: #1b1b1b !important; }',
        '.message-content { background: #1b1b1b !important; }',
        '.bodyDecrypted blockquote { color: #555 !important; }',
        '.bordered, .bordered-container, .breadcrumb-container { border-color: #1a1a1a !important; }',
        '.fill-global-grey { fill: #777; }',
        '.starbutton { fill: #777 !important; }',

        '#pm_composer .composer .composerHeader-container { color: #ddd !important; background: 111 !important; }',
        '#pm_composer .composer { background: #212121 !important; }',
        '#pm_composer .focus.pm-field-icon-container, #pm_composer .pm-field-icon-container:focus, #pm_composer .pm-field-icon-container:focus-within, #pm_composer .pm-field.focus, #pm_composer .pm-field:focus, #pm_composer .pm-field:focus-within { background: #777 !important; }',
        '.composerInputMeta-overlay { background: #212121 !important; }',
        '.composerInputMeta-overlay-fakefield, .composerInputMeta-autocomplete { background: #777 !important; }',
        '#pm_composer label { color: #ddd !important; }',

        '.angular-squire-iframe body { background: red; }',

        '.pm_panel-login__inner input[type=text], .pm_panel-login__inner .pm-field-icon-container { background: #777 !important; border: 1px solid #444 !important; }',
        '#pm_footer, #pm_footer * { color: #555 !important; }',
    '}'
]);
