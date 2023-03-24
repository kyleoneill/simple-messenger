import axios from "axios";

export function getRelationships(): Promise<any> {
    return axios.get("/api/relationships")
        .then(function (response) {
            return response;
        })
        .catch(function (error) {
            return error;
        });
}

export async function getFriends(): Promise<string[]> {
    let friends = [];
    let response = await getRelationships();
    if(response.status === 200) {
        friends = response.data;
    }
    return friends;
}
