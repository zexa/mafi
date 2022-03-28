import { Text, Input, Spacer, Button } from "@nextui-org/react";
import Router from 'next/router';
import getConfig from 'next/config';
import MafiClient from '../../clients/mafi_client.ts';
import cookies from 'next-cookies';

export async function getServerSideProps(context) {
    const mafiClient = new MafiClient(getConfig().serverRuntimeConfig.apiUrl);

    if (typeof context.query.name === 'string') {
        const lobby = await mafiClient.createLobby(context.query.name, cookies(context).SESSION);

        return {
            props: {
                lobbyId: lobby.uuid,
            },
        };
    }

    return {props: {}};
}

function CreateLobby(props) {
    if (typeof props.lobbyId === 'string') {
        Router.push(`/lobby/${props.lobbyId}`);
    }

    const clicky = function() {
        const name = document.getElementById('name').value;
        Router.push(`?name=${name}`);
    }

    return (
        <div>
            <Text h1>Create Lobby</Text>
            <Input id="name" label="Name" placeholder="My random lobby name" />
            <Spacer y={1}/>
            <Button onClick={clicky}>Create Lobby</Button>
        </div>
    );
}

export default CreateLobby;