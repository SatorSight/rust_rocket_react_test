import React from 'react';
import ReactDOM from 'react-dom';
import 'whatwg-fetch';

class IndexPage extends React.Component {
    constructor(props){
        super(props);

        this.state = {
            users: [],
            stagings: [],

            new_user_name: '',
            new_staging_name: '',
        };
    }

    componentDidMount(){
        fetch('/')
            //todo set content-type on backend
            .then(r => r.json())
            .then(r => {
                this.setState({
                    users: r.users,
                    stagings: r.stagings
                });
            })
    }

    addUser = () => {
        fetch('http://localhost:8000/add_user', {
            method: 'POST',
            headers: {
                'Accept': 'application/json',
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({ name: this.state.new_user_name })
        })
            .then(r => r.json())
            .then(r => console.log(r))
        // TODO: add this user on front too
            // .then(() => this.setState({ users: [{name: }] }))
    };

    addStaging = () => {
        fetch('http://localhost:8000/add_staging', {
            method: 'POST',
            headers: {
                'Accept': 'application/json',
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({ name: this.state.new_staging_name })
        })
            .then(r => r.json())
            .then(r => console.log(r))
    };

    changeNewUserName = e => this.setState({ new_user_name: e.target.value });
    changeNewStagingName = e => this.setState({ new_staging_name: e.target.value });

    render() {
        return (
            <div className="container">
                <div className="users">
                    {this.state.users.map(user =>
                        <div key={user.id}>{user.name}</div>
                    )}
                </div>
                <div className="stagings">
                    {this.state.stagings.map(staging =>
                        <div key={staging.id}>{staging.name}</div>
                    )}
                </div>
                <input type="text" value={this.state.new_user_name} onChange={this.changeNewUserName}/>
                <button onClick={this.addUser}>add user</button>
                <input type="text" value={this.state.new_staging_name} onChange={this.changeNewStagingName}/>
                <button onClick={this.addStaging}>add staging</button>
            </div>
        );
    }
}

ReactDOM.render(<IndexPage />, document.getElementById('root'));
