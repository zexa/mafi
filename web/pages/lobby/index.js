import { Container, Row, Col, Text, Link, Spacer, Button } from '@nextui-org/react';
import getConfig from 'next/config';
import MafiClient from '../../clients/mafi_client.ts';
import cookies from 'next-cookies';

export async function getServerSideProps(context) {
    const mafiClient = new MafiClient(getConfig().serverRuntimeConfig.apiUrl)
    const sessionId = cookies(context).SESSION;
    console.log('Getting lobbies with session', sessionId);
    let lobbies = await mafiClient.getLobbies(sessionId);
    console.log('Got lobbies', lobbies);
    lobbies = lobbies.map(lobby => {
        return {
            name: lobby.name,
            id: lobby.uuid,
        };
    });

    return {
        props: {
            lobbies: lobbies,
        },
    };
}

function Lobbies(props) {
    return (
        <div>
            <Text h1>Lobbies</Text>
            <Container gap={0}>
                <Row gap={0}>
                    <Col><Text h6>Lobby name</Text></Col>
                    <Col><Text h6>Lobby Id</Text></Col>
                    <Col><Text h6>Join Lobby</Text></Col>
                </Row>
                {props.lobbies.map(lobby => (
                    <Row>
                        <Col><Text>{lobby.name}</Text></Col>
                        <Col><Text>{lobby.id}</Text></Col>
                        <Col><Link href={`/lobby/${lobby.id}`}>Join {lobby.name}</Link></Col>
                    </Row>
                ))}
            </Container>
            <Spacer y={1}/>
            <Link href="/lobby/create"><Button>Create Lobby</Button></Link>
        </div>
    );
}

export default Lobbies;
