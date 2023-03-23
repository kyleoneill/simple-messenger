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
