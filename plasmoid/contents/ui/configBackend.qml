import QtQuick
import QtQuick.Controls
import QtQuick.Layouts 1.12
import org.kde.kirigami as Kirigami

Kirigami.FormLayout {
    id: page

    property int cfg_tlyricMode: 0
    property alias cfg_verbose: verbose.checked
    property alias cfg_disabledPlayers: disabledPlayers.text
    property alias cfg_enabledLyricProviders: enabledLyricProviders.text
    property alias disabledFolders: disabledFolders.text

    Label {
        text: i18n('Lyric translation mode:')
    }

    ComboBox {
        id: tlyricMode
        textRole: 'label'
        model: [
            {
                'label': i18n('Show original lyric only'),
                'value': 0
            },
            {
                'label': i18n('Show translation only'),
                'value': 1
            },
            {
                'label': i18n('Original lyric | translation'),
                'value': 2
            },
            {
                'label': i18n('Translation | original lyric'),
                'value': 3
            }
        ]
        onCurrentIndexChanged: cfg_tlyricMode = model[currentIndex]['value']

        Component.onCompleted: {
            for (var i = 0; i < model.length; i++) {
                if (model[i]['value'] == plasmoid.configuration.tlyricMode) {
                    tlyricMode.currentIndex = i
                }
            }
        }

        property string currentVal: model[currentIndex]['value']
    }

    CheckBox {
        id: verbose
        text: i18n("Show detailed logs in the journal")
    }

    TextField {
        id: disabledPlayers
        Kirigami.FormData.label: i18n("Disabled players (comma separated):")
        placeholderText: i18n("firefox,chromium,plasma-browser-integration,kdeconnect")
    }

    TextField {
        id: enabledLyricProviders
        Kirigami.FormData.label: i18n("Enabled lyric providers (comma separated):")
        placeholderText: i18n("file,netease,yesplaymusic")
    }

    Label {
        text: i18n('(Available providers: file, netease, yesplaymusic)')
    }

    TextArea {
        id: disabledFolders
        Kirigami.FormData.label: i18n("Disabled folders (one per line):")
        placeholderText: i18n("/home/user/Music/lyric\n/home/user/Music/lyric2")
    }


}
