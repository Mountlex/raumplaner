


export class AuthService {
  constructor(url) {
    this.url = url
  }

  async login(username, password) {
    const res = await fetch(this.url + '/login', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({
        client_id: username,
        client_secret: password
      })
    });
    const resJson = await res.json();
    return resJson.access_token;
  }
};


