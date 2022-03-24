class MafiClient {
    apiUrl: string;

    constructor(apiUrl: string) {
        this.apiUrl = apiUrl
    }

    async getRoles(): Promise<string[]> {
        const result = await fetch(`${this.apiUrl}/roles`);

        return result.json();
    }
}
