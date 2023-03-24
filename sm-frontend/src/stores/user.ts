import {displayToast} from "./errorToast";
import {createUser, login, logout} from "../api/user";
import axios from "axios";
import {defineStore} from "pinia";

type User = {
    username: String,
    token: String
}

export const useUserStore = defineStore('userData', {
    state: () => {
        return {
            username: '',
            friends: []
        }
    },
    actions: {
        updateUsername(newUsername: string) {
            this.username = newUsername;
        },
        setFriends(friendsList: string[]) {
            this.friends = friendsList;
        }
    }
});

export function getUserFromStorage(): User {
    let storedName = window.localStorage.getItem("username");
    let storedToken = window.localStorage.getItem("token");
    if (storedName != null && storedToken != null) {
        return { username: storedName, token: storedToken }
    }
    else {
        return { username: '', token: '' }
    }
}

export function tryCreateUser(enteredUsername: string, enteredPassword: string, confirmPassword: string, setUserInfo: (enteredUsername: string, token: string) => void) {
    if(enteredPassword !== '' && enteredPassword !== confirmPassword) {
        displayToast("error", "Entered passwords do not match.");
    }
    else if(enteredUsername !== '' && enteredPassword !== '' && enteredPassword === confirmPassword) {
        createUser(enteredUsername, enteredPassword).then(response => {
            if(response.status === 201) {
                tryLogIn(enteredUsername, enteredPassword, setUserInfo);
            }
            else if(response.status === 409) {
                displayToast("error", "That username is already in use.");
            }
            else {
                displayToast("error", "Unknown error when creating a new user.");
            }
        });
    }
}

export function tryLogIn(enteredUsername: string, enteredPassword: string, setUserInfo: (enteredUsername: string, token: string) => void ) {
    if(enteredUsername !== '' && enteredPassword !== '') {
        login(enteredUsername, enteredPassword).then(response => {
            if(response.status === 201) {
                setLocalStorage(enteredUsername, response.data.token);
                setUserInfo(enteredUsername, response.data.token);
            }
            else {
                displayToast("error", "Invalid username/password.");
                return { username: enteredUsername, token: response.data.token }
            }
        });
    }
}

function setLocalStorage(username: string, token: string) {
    localStorage.setItem("username", username);
    localStorage.setItem("token", token);
}

export function tryGetLocalStorage(setUserInfo: (enteredUsername: string, token: string) => void) {
    let username = localStorage.getItem("username");
    let token = localStorage.getItem("token");
    if(username !== null && token !== null) {
        setUserInfo(username, token);
    }
}

export function tryLogout() {
    logout().then(response => {
        if(response.status !== 200) {
            displayToast("error", "Failed to logout successfully.");
        }
        localStorage.removeItem("username");
        localStorage.removeItem("token");
        axios.defaults.headers.common["Authorization"] = "";
        location.reload();
    });
}