type User = {
    username: String,
    token: String
}

export function getUser() {
    let storedName = window.localStorage.getItem("username");
    let storedToken = window.localStorage.getItem("token");
    if (storedName != null && storedToken != null) {
        return { username: storedName, token: storedToken }
    }
    else {
        return { username: '', token: '' }
    }
}