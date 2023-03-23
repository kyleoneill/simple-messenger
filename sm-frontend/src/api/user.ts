import axios from "axios";

export function login(enteredUsername: string, enteredPassword: string): Promise<any> {
    return axios.post("/api/users/auth", {
        username: enteredUsername,
        password: enteredPassword
    })
    .then(function (response) {
        return response;
    })
    .catch(function (error) {
        // TODO: HANDLE ERROR STATE?
        return error;
    });
}

export function createUser(enteredUsername: string, enteredPassword: string): Promise<any> {
    return axios.post("/api/users", {
        username: enteredUsername,
        password: enteredPassword
    })
    .then(function (response) {
        return response;
    })
    .catch(function (error) {
        return error;
    });
}

export function logout(): Promise<any> {
    return axios.post("/api/users/logout")
        .then(function (response) {
            return response;
        })
        .catch(function (error) {
            return error;
        });
}
