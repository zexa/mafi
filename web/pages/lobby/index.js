import { Container, Row, Col } from '@nextui-org/react';
import { Text } from '@nextui-org/react';
import { Link } from '@nextui-org/react';

function Lobbies() {
    return (
        <div>
            <Text h1>Lobbies</Text>
            <Container gap={0}>
                <Row gap={0}>
                    <Col><Text h6>Lobby name</Text></Col>
                    <Col><Text h6>Lobby Id</Text></Col>
                    <Col><Text h6>Join Lobby</Text></Col>
                </Row>
                <Row>
                    <Col><Text>pirmas blynas</Text></Col>
                    <Col><Text>random-uuid-here</Text></Col>
                    <Col><Link href="#">Join pirmas blynas</Link></Col>
                </Row>
            </Container>
        </div>
    );
}

export default Lobbies;
