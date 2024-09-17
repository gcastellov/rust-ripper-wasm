CookieConsent.run({
    categories: {
        necessary: {
            enabled: true,
            readOnly: true
        },
        analytics: {
            enabled: true,
            readOnly: false
        }
    },

    language: {
        default: 'en',
        translations: {
            en: {
                consentModal: {
                    title: 'Cookies?',
                    description: 'By clicking Accept All”, you agree to the storing of cookies on your device to enhance site navigation and to analyze site usage.',
                    acceptAllBtn: 'Accept all',
                    acceptNecessaryBtn: 'Reject all',
                    showPreferencesBtn: 'Manage Individual preferences'
                },
                preferencesModal: {
                    title: 'Manage cookie preferences',
                    acceptAllBtn: 'Accept all',
                    acceptNecessaryBtn: 'Reject all',
                    savePreferencesBtn: 'Accept current selection',
                    closeIconLabel: 'Close modal',
                    sections: [
                        {
                            title: 'Strictly Necessary cookies',
                            description: 'These cookies are essential for the proper functioning of the website and cannot be disabled.',
                            linkedCategory: 'necessary'
                        },
                        {
                            title: 'Performance and Analytics',
                            description: 'These cookies collect information about how you use our website. All of the data is anonymized and cannot be used to identify you.',
                            linkedCategory: 'analytics'
                        }
                    ]
                }
            }
        }
    }
});