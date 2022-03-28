import { Container, Row, Col, Link, Card } from '@nextui-org/react';
import { Text } from '@nextui-org/react';
import { Button } from '@nextui-org/react';
import { Spacer } from '@nextui-org/react';
import getConfig from 'next/config';
import MafiClient from '../../../clients/mafi_client.ts';
import { useRouter } from 'next/router'
import cookies from 'next-cookies';

export async function getServerSideProps(context) {
    const mafiClient = new MafiClient(getConfig().serverRuntimeConfig.apiUrl)
    const sessionId = cookies(context).SESSION;
    let errs = [];

    if (typeof context.query.joinGame !== 'undefined') {
        await mafiClient.joinLobby(context.params.lobbyId, sessionId);
    }

    if (typeof context.query.startGame !== 'undefined') {
        const lobbyResponse = await mafiClient.createGame(context.params.lobbyId, sessionId);
        if (typeof lobbyResponse.message === 'string') {
            errs.push(lobbyResponse.message);
        }
    }

    if (typeof context.query.addRole === 'string') {
        await mafiClient.addLobbyRole(context.params.lobbyId, context.query.addRole, sessionId)
    }

    if (typeof context.query.removeRole === 'string') {
        await mafiClient.removeLobbyRole(context.params.lobbyId, context.query.removeRole, sessionId)
    }

    const roles = await mafiClient.getRoles();
    const lobby = await mafiClient.getLobby(context.params.lobbyId, sessionId);
    
    const playerAmount = lobby.players.length;
    const lobbyRolesAmount = lobby.roles.length;
    const availableRolesAmount = roles.length;
    let tableSize = playerAmount > lobbyRolesAmount ? playerAmount : lobbyRolesAmount;
    tableSize = tableSize > availableRolesAmount ? tableSize : availableRolesAmount;

    let table = [];
    for (let i = 0; i < tableSize; i++) {
        const player = lobby.players[i] ?? null;

        table[i] = {
            playerName: player === null ? '' : player.name,
            lobbyRole: lobby.roles[i] ?? '',
            availableRole: roles[i] ?? '',
        };
    }

    let games = await mafiClient.getGames(context.params.lobbyId, sessionId);
    games = games.map(game => game.uuid);

    return {
        props: {
            errors: errs,
            lobbyName: lobby.name,
            lobbyId: lobby.uuid,
            lobbyOwnerName: lobby.owner.name,
            table: table,
            isOwner: sessionId === lobby.owner.secret,
            games: games,
        }
    }
}

function IndividualLobby(props) {
    const gameButton = props.isOwner 
        ? (<Link href="?startGame"><Button >Start Game</Button></Link>)
        : (<Link href="?joinGame"><Button>Join Lobby</Button></Link>)
    ;

    return (
        <div>
            {props.errors.map((err) => 
                <Card color="error">{err}</Card>
            )}
            <Text h1>Lobby</Text>
            <Text h2>{props.lobbyName} by {props.lobbyOwnerName}</Text>
            <Spacer y={1}/>
            <Container gap={0}>
                <Row>
                    <Col>
                        <Text h6>Joined Users</Text>
                    </Col>
                    <Col>
                        <Text h6>Lobby Roles</Text>
                    </Col>
                    <Col>
                        <Text h6>Available Roles</Text>
                    </Col>
                </Row>
                { props.table.map((row) =>
                    <Row>
                        <Col><Text>{row.playerName}</Text></Col>
                        <Col><Link color="error" href={`?removeRole=${row.lobbyRole}`}>{row.lobbyRole}</Link></Col>
                        <Col><Link href={`?addRole=${row.availableRole}`}>{row.availableRole}</Link></Col>
                    </Row>
                )}
            </Container>
            <Spacer y={1}/>
            {gameButton}
            <Spacer y={1}/>
            <Container gap={0}>
                <Row>
                    <Col>
                        <Text h6>Game id</Text>
                    </Col>
                    <Col>
                        <Text h6>Join Game</Text>
                    </Col>
                </Row>
                {props.games.map(gameId => (
                    <Row>
                        <Col>
                            <Text>{gameId}</Text>
                        </Col>
                        <Col>
                            <Link href={`/lobby/${props.lobbyId}/game/${gameId}`}>Join {gameId}</Link>
                        </Col>
                    </Row>
                ))}
            </Container>
        </div>
    );
}

export default IndividualLobby;
