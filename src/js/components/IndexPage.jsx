import React from 'react';
import ReactDOM from 'react-dom';
import 'whatwg-fetch';

class IndexPage extends React.Component {
    constructor(props){
        super(props);

        this.state = {
            users: [],
            stagings: [],
            users_stagings: [],

            new_user_name: '',
            new_staging_name: '',
        };
    }

    componentDidMount(){
        this.fetchFor('all')
            .then(r =>
                this.setState({
                    users: r.users,
                    stagings: r.stagings,
                    users_stagings: r.users_stagings,
                }))
    }

    //todo set content-type on backend
    fetchFor = (path, data = null, method = 'GET') =>
        fetch(`http://localhost:8000/api/${path}`, {
            method: method,
            headers: {
                'Accept': 'application/json',
                'Content-Type': 'application/json'
            },
            body: data && JSON.stringify(data)
        })
            .then(r => r.json());

    addUser = () =>
        this.fetchFor('add_user', { name: this.state.new_user_name })
            .then(r => this.setState({ users: [r, ...this.state.users] }));

    addStaging = () =>
        this.fetchFor('add_staging', { name: this.state.new_staging_name })
            .then(s => this.setState({ stagings: [s, ...this.state.stagings] }));

    assignStagingToUser = (staging_id, user_id) =>
        this.fetchFor('assign_staging_to_user', { user_id: user_id, staging_id: staging_id }, 'UPDATE');

    deleteStaging = id => this.fetchFor('staging', { staging_id: id }, 'DELETE')
        .then(() => this.setState({ stagings: this.state.stagings.filter(s => s.id !== id) }));

    deleteUser = id => this.fetchFor('user', { user_id: id }, 'DELETE')
        .then(() => this.setState({ users: this.state.users.filter(s => s.id !== id) }));

    onStaging = (user, staging) =>
        this.state.users_stagings.find(us => us.user_id === user.id && us.staging_id === staging.id);

    changeNewUserName = e => this.setState({ new_user_name: e.target.value });
    changeNewStagingName = e => this.setState({ new_staging_name: e.target.value });

    toggleBusy = staging =>
        this.fetchFor('staging', { staging_id: staging.id }, 'PATCH')
            .then(() =>
                this.setState({ stagings: this.state.stagings.map(s => {
                    if(s.id === staging.id)
                        s.busy = !s.busy;
                    return s;
                })}));

    options = staging =>
        this.state.users.map(u =>
            <option key={u.id} selected={this.onStaging(u, staging)} value={u.id}>{u.name}</option>);

    render() {
        return (
            <div className="container">
                {this.state.stagings.map(s =>
                    <div key={s.id} className="row">
                        <div className="staging_name">{s.name}</div>
                        <div className="staging_user">
                            <select onChange={e => this.assignStagingToUser(s.id, +e.target.value)}>
                                {this.options(s)}
                            </select>
                        </div>
                        <div className="toggleBusy">
                            <button onClick={() => this.toggleBusy(s)} style={s.busy ? {color: 'red'} : {color: 'green'}}>
                                {s.busy ? 'Busy' : 'Free'}
                            </button>
                        </div>
                    </div>
                )}
                <br/>
                <br/>
                <div>USERS</div>
                <div className="users">
                    {this.state.users.map(user =>
                        <div key={user.id}>
                            <span>{user.name}</span>
                            <span onClick={() => this.deleteUser(user.id)}>x</span>
                        </div>
                    )}
                </div>
                <br/>
                <br/>
                <div>STAGINGS</div>
                <div className="stagings">
                    {this.state.stagings.map(staging =>
                        <div key={staging.id}>
                            <span>{staging.name}</span>
                            <span onClick={() => this.deleteStaging(staging.id)}>x</span>
                        </div>
                    )}
                </div>
                <input type="text" value={this.state.new_user_name} onChange={this.changeNewUserName}/>
                <button onClick={this.addUser}>add user</button>
                <input type="text" value={this.state.new_staging_name} onChange={this.changeNewStagingName}/>
                <button onClick={this.addStaging}>add staging</button>
                {/*<button onClick={() => this.assignStagingToUser(1,1)}>add first staging to first</button>*/}
            </div>
        );
    }
}

ReactDOM.render(<IndexPage />, document.getElementById('root'));
