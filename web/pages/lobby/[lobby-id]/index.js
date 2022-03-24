import { Container, Row, Col, Link } from '@nextui-org/react';
import { Text } from '@nextui-org/react';
import { Button } from '@nextui-org/react';
import { Spacer } from '@nextui-org/react';

function IndividualLobby(props) {
    return (
        <div>
            <Text h1>Lobby</Text>
            <Text h2>Pirmas blynas by Tomas</Text>
            <Spacer y={1}/>
            <Container gap={0}>
                <Row>
                    <Col>
                        <Text h6>Joined Users</Text>
                    </Col>
                    <Col>
                        <Text h6>Available Roles</Text>
                    </Col>
                    <Col>
                        <Text h6>Role selection</Text>
                    </Col>
                </Row>
                <Row>
                    <Col>
                        <Text>Augustinas</Text>
                    </Col>
                    <Col>
                        <Link color="error" href="#">Godfather</Link>
                    </Col>
                    <Col>
                        <Link href="#">Godfather</Link>
                    </Col>
                </Row>
                <Row>
                    <Col>
                        <Text>Gerda</Text>
                    </Col>
                    <Col>
                        <Link color="error" href="#">Escort</Link>
                    </Col>
                    <Col>
                        <Link href="#">Mafia Goon</Link>
                    </Col>
                </Row>
                <Row>
                    <Col>
                        <Text>Kostas</Text>
                    </Col>
                    <Col>
                        <Link color="error" href="#">Mafia Goon</Link>
                    </Col>
                    <Col>
                        <Link href="#">Escort</Link>
                    </Col>
                </Row>
                <Row>
                    <Col></Col>
                    <Col></Col>
                    <Col>
                        <Link href="#">Townie</Link>
                    </Col>
                </Row>
            </Container>
            <Spacer y={1}/>
            <Button>Start Game</Button>
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
                <Row>
                    <Col>
                        <Text>random-uuid-here</Text>
                    </Col>
                    <Col>
                        <Link href="#">Join random-uuid-here</Link>
                    </Col>
                </Row>
            </Container>
        </div>
    );
}

export default IndividualLobby;
