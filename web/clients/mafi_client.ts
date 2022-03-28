import { responseSymbol } from "next/dist/server/web/spec-compliant/fetch-event";

interface User {
    readonly name: string,
    readonly secret: string,
}

interface Player {
    readonly user: User,
    readonly role: string,
}

interface Game {
    readonly uuid: string,
    readonly lobby_uuid: string,
    readonly players: Array<Player>,
    readonly game_status: string,
}

interface Lobby {
    readonly uuid: string,
    readonly name: string,
    readonly owner: User,
    readonly roles: Array<string>,
    readonly players: Array<User>,
}

export default class MafiClient {
    apiUrl: string;

    constructor(apiUrl: string) {
        this.apiUrl = apiUrl
    }

    async getRoles(): Promise<string[]> {
        const result = await fetch(`${this.apiUrl}/roles`);

        return result.json();
    }

    async registerUser(userName: string): Promise<User> {
        const result = await fetch(
            `${this.apiUrl}/user`,
            {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify({name: userName}),
            }
        );

        return result.json()
    }

    async getLobbies(sessionId: string): Promise<Array<Lobby>> {
        const result = await fetch(
            `${this.apiUrl}/lobbies`,
            {
                headers: {
                    Cookie: `SESSION=${sessionId}`,
                },
            }
        );

        return result.json();
    }

    async createLobby(lobbyName: string, sessionId: string): Promise<Lobby> {
        const result = await fetch(
            `${this.apiUrl}/lobby`,
            {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                    Cookie: `SESSION=${sessionId}`,
                },
                body: JSON.stringify({name: lobbyName})
            }
        );

        return result.json();
    }

    async joinLobby(lobbyId: string, sessionId: string): Promise<any> {
        const result = await fetch(
            `${this.apiUrl}/lobby/${lobbyId}/join`,
            {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                    Cookie: `SESSION=${sessionId}`,
                },
            }
        );

        return;
    }

    async createGame(lobbyId: string, sessionId: string): Promise<any> {
        const result = await fetch(
            `${this.apiUrl}/lobby/${lobbyId}/game`,
            {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                    Cookie: `SESSION=${sessionId}`,
                },
            }
        );

        return result.json();
    }

    async getLobby(lobbyId: string, sessionId: string): Promise<Lobby> {
        const result = await fetch(
            `${this.apiUrl}/lobby/${lobbyId}`,
            {
                headers: {
                    Cookie: `SESSION=${sessionId}`,
                },
            }
        );

        return result.json();
    }

    async addLobbyRole(lobbyId: string, role: string, sessionId: string): Promise<any> {
        const response = await fetch(
            `${this.apiUrl}/lobby/${lobbyId}/role`,
            {
                body: JSON.stringify({ role: role }),
                method: 'POST',
                headers: {
                    "Content-Type": 'application/json',
                    Cookie: `SESSION=${sessionId}`,
                },
            }
        );

        return
    }

    async removeLobbyRole(lobbyId: string, role: string, sessionId: string): Promise<any> {
        const response = await fetch(
            `${this.apiUrl}/lobby/${lobbyId}/role`,
            {
                body: JSON.stringify({ role: role }),
                method: 'DELETE',
                headers: {
                    "Content-Type": 'application/json',
                    Cookie: `SESSION=${sessionId}`,
                },
            }
        );

        return
    }

    async getGames(lobbyId: string, sessionId: string): Promise<Array<Player>> {
        const result = await fetch(
            `${this.apiUrl}/lobby/${lobbyId}/games`,
            {
                headers: {
                    Cookie: `SESSION=${sessionId}`,
                },
            }
        );

        return result.json();
    }

    async getGame(lobbyId: string, gameId: string, sessionId: string): Promise<Game>  {
        const result = await fetch(
            `${this.apiUrl}/lobby/${lobbyId}/game/${gameId}`,
            {
                headers: {
                    Cookie: `SESSION=${sessionId}`,
                },
            }
        );

        return result.json();
    }
}
