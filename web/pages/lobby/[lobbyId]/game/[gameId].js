import { Link, Text } from '@nextui-org/react';
import getConfig from 'next/config';
import MafiClient from '../../../../clients/mafi_client.ts';
import cookies from 'next-cookies';

export async function getServerSideProps(context) {
    const mafiClient = new MafiClient(getConfig().serverRuntimeConfig.apiUrl);
    const sessionId = cookies(context).SESSION;

    const game = await mafiClient.getGame(context.params.lobbyId, context.params.gameId, sessionId);
    const gameRoles = game.players.map(player => player.role);
    const currentPlayer = game.players.find(player => player.user.secret === sessionId) ?? null;
    const playerRole = currentPlayer !== null
        ? currentPlayer.role
        : 'Admin';
    const isAdmin = playerRole === 'Admin';
    const playerRoles = isAdmin
        ?  game.players.map(player => {
            return {
                name: player.user.name,
                role: player.role,
            };
        })
        : [];

    return {
        props: {
            gameRoles: gameRoles,
            playerRole: playerRole,
            isAdmin: isAdmin,
            playerRoles,
        },
    };
}

function IndividualGame(props) {
    const playerRoles = props.isAdmin
        ? JSON.stringify(props.playerRoles)
        : ''

    return (
        <div>
            <Text h1>Game</Text>
            <Text>Roles in the game: {props.gameRoles.join(', ')}</Text>
            <Text h2>Your role: {props.playerRole}</Text>
            {playerRoles}
        </div>
    );
}

export default IndividualGame; 
