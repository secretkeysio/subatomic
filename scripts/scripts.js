/**
 *  script.js
 *
 *  A UserScript for protonmail.com to extract data for better push notifications.
 *  These generally aim to mimic those emitted by Mail.app.
 **/

;(function() {
    // Track when document.title is updated. We could do this with KVO on the native side, but...
    // well, I don't feel like writing support for that right now, so we'll just use a MutationObserver
    // and hoist it upwards.
    document.addEventListener('DOMContentLoaded', function() {
    var target = document.querySelector('head > title');
    var observer = new MutationObserver(function(mutation) { 
        mutation.forEach(function(m) { 
            webkit.messageHandlers.updateTitle.postMessage(m.target.textContent);
        });
    });

    observer.observe(target, {
        subtree: true,
        characterData: true,
        childList: true
    });
    }, false);

    var open = XMLHttpRequest.prototype.open;
    
    XMLHttpRequest.prototype.open = function() {
        console.log('Path: ' + arguments[1]);

        this.addEventListener('load', function() {
            if(typeof window.webkit.messageHandlers.notify === 'undefined')
                return;
            
            // Listening to /api/conversations/ would give more information to work with (e.g, attachment names)
            // but there's an argument for not leaking that in a push notification. We'll instead just detect
            // if there are any attachments, and shove in an emoji if so.
            //
            // This also fires first, so this is slightly faster than waiting on /api/conversations/.
            if(this.responseURL.indexOf('https://beta.protonmail.com/api/events/') !== 0)
                return;

            try {
                var data = JSON.parse(this.responseText);
            } catch(e) {
                return;
            }

            if(typeof data.Messages === 'undefined')
                return;

            data.Messages.forEach(function(messageContainer) {
                if(typeof messageContainer.Message === 'undefined')
                    return;

                var message = messageContainer.Message;

                // Events that don't need push notifications
                if(typeof message.SenderName === 'undefined' || typeof message.Subject === 'undefined')
                    return;

                // Don't notify on read messages (e.g, if they read it via the mobile app already, or this is a
                // view updating from a disconnected state)
                if(typeof message.Unread !== 'undefined' && !message.Unread)
                    return;

                var msg = {
                    title: message.Subject,
                    body: message.SenderName
                };

                if(typeof message.HasAttachment && message.HasAttachment)
                    msg.body = '📎 ' + msg.body;

                window.webkit.messageHandlers.notify.postMessage(JSON.stringify(msg));
            });
        });

        open.apply(this, arguments);
    };
})();
