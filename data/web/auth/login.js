// UM-OIC Login JavaScript

document.addEventListener('DOMContentLoaded', function() {
    const loginForm = document.getElementById('loginForm');
    const errorMessage = document.getElementById('errorMessage');
    const loginBtn = document.querySelector('.login-btn');

    // Check for OAuth2 parameters in URL
    const urlParams = new URLSearchParams(window.location.search);
    const clientId = urlParams.get('client_id');
    const redirectUri = urlParams.get('redirect_uri');
    const responseType = urlParams.get('response_type');
    const scope = urlParams.get('scope');
    const state = urlParams.get('state');

    // If this is an OAuth2 authorization request, show different UI
    if (clientId && redirectUri && responseType) {
        showOAuth2Flow();
    }

    loginForm.addEventListener('submit', async function(e) {
        e.preventDefault();

        const email = document.getElementById('email').value;
        const password = document.getElementById('password').value;

        if (!email || !password) {
            showError('Bitte E-Mail und Passwort eingeben');
            return;
        }

        try {
            setLoading(true);
            hideError();

            // Attempt login
            const loginResult = await performLogin(email, password);

            if (loginResult.success) {
                // Store the access token for admin service access
                if (loginResult.token) {
                    localStorage.setItem('auth_token', loginResult.token);
                }

                // If this is an OAuth2 flow, proceed with authorization
                if (clientId && redirectUri && responseType) {
                    await handleOAuth2Authorization(loginResult.token);
                } else {
                    // Check if there's a redirect parameter
                    const redirectUrl = urlParams.get('redirect');
                    if (redirectUrl) {
                        // Add token to URL and redirect
                        const url = new URL(redirectUrl);
                        url.searchParams.set('token', loginResult.token);
                        window.location.href = url.toString();
                    } else {
                        // Regular login - redirect to management interface
                        window.location.href = 'http://localhost:8444/';
                    }
                }
            } else {
                showError(loginResult.error || 'Login fehlgeschlagen');
            }
        } catch (error) {
            console.error('Login error:', error);
            showError('Verbindungsfehler. Bitte versuchen Sie es erneut.');
        } finally {
            setLoading(false);
        }
    });

    async function performLogin(email, password) {
        try {
            // Since we don't have a direct login endpoint in the current API,
            // we'll simulate the login process and check if the credentials are valid
            // by attempting to get user info or checking against the OAuth2 flow

            const response = await fetch('/api/auth/login', {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify({
                    email: email,
                    password: password
                })
            });

            if (response.ok) {
                const tokenData = await response.json();
                return { success: true, token: tokenData.access_token };
            } else {
                const errorData = await response.json().catch(() => ({}));
                return { success: false, error: errorData.error_description || 'Ungültige Anmeldedaten' };
            }
        } catch (error) {
            return { success: false, error: 'Verbindungsfehler' };
        }
    }

    async function handleOAuth2Authorization(accessToken) {
        try {
            // Build authorization URL
            const authUrl = new URL('/oauth2/authorize', window.location.origin);
            authUrl.searchParams.set('response_type', responseType);
            authUrl.searchParams.set('client_id', clientId);
            authUrl.searchParams.set('redirect_uri', redirectUri);
            if (scope) authUrl.searchParams.set('scope', scope);
            if (state) authUrl.searchParams.set('state', state);

            // Redirect to authorization endpoint with user context
            window.location.href = authUrl.toString();
        } catch (error) {
            showError('OAuth2 Autorisierung fehlgeschlagen');
        }
    }

    function showOAuth2Flow() {
        // Add OAuth2 context information to the UI
        const header = document.querySelector('.header');
        const oauth2Info = document.createElement('div');
        oauth2Info.className = 'oauth2-info';
        oauth2Info.innerHTML = `
            <p><small>Anmeldung für Anwendung: <strong>${clientId}</strong></small></p>
            <p><small>Berechtigung: <strong>${scope || 'openid'}</strong></small></p>
        `;
        header.appendChild(oauth2Info);

        // Add OAuth2 specific styles
        const style = document.createElement('style');
        style.textContent = `
            .oauth2-info {
                background: #e6fffa;
                border: 1px solid #81e6d9;
                border-radius: 6px;
                padding: 10px;
                margin-bottom: 20px;
                font-size: 0.8rem;
                color: #234e52;
            }
        `;
        document.head.appendChild(style);
    }

    function showError(message) {
        errorMessage.textContent = message;
        errorMessage.style.display = 'block';
    }

    function hideError() {
        errorMessage.style.display = 'none';
    }

    function setLoading(loading) {
        if (loading) {
            loginBtn.classList.add('loading');
            loginBtn.disabled = true;
            loginBtn.textContent = 'Anmelden...';
        } else {
            loginBtn.classList.remove('loading');
            loginBtn.disabled = false;
            loginBtn.textContent = 'Anmelden';
        }
    }

    // Auto-focus on email field
    document.getElementById('email').focus();
});