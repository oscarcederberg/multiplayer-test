package;

import flixel.FlxG;
import flixel.FlxSprite;
import flixel.FlxState;
import flixel.FlxSubState;
import flixel.text.FlxText;
import flixel.ui.FlxButton;
import flixel.util.FlxColor;

class HostState extends FlxSubState
{
	var lobbyInfoText:FlxText;

	override function create()
	{
		super.create();

		var background = new FlxSprite();
		background.makeGraphic(Std.int(3 / 4 * FlxG.width), Std.int(2 / 3 * FlxG.height), FlxColor.BLACK);
		background.screenCenter(XY);

		var hostTitle = new FlxText(0, 0, 0, "Host Game", 32);
		hostTitle.screenCenter(XY).y -= 80;

		lobbyInfoText = new FlxText(0, 0, 0, "Lobby code: #####", 16);
		lobbyInfoText.addFormat(new FlxTextFormat(FlxColor.GREEN, true), "Lobby code:".length, "Lobby code: #####".length);
		lobbyInfoText.screenCenter(XY);

		var exitButton = new FlxButton(0, 0, "Cancel", onExitButtonClicked);
		exitButton.screenCenter(XY).y += 64;

		add(background);
		add(hostTitle);
		add(lobbyInfoText);
		add(exitButton);
	}

	private function onExitButtonClicked()
	{
		close();
	}
}

class MenuState extends FlxState
{
	override public function create()
	{
		super.create();

		var hostButton = new FlxButton(0, 0, "Host", onHostClicked);
		hostButton.screenCenter(XY).y -= 12;

		var joinButton = new FlxButton(0, 0, "Join", onJoinClicked);
		joinButton.screenCenter(XY).y += 12;

		add(hostButton);
		add(joinButton);
	}

	override public function update(elapsed:Float)
	{
		super.update(elapsed);
	}

	private function onHostClicked()
	{
		openSubState(new HostState());
	}

	private function onJoinClicked() {}
}
