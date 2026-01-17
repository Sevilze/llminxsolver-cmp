package com.llminxsolver.viewmodel

import com.llminxsolver.data.MegaminxState
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.flow.update

class MegaminxViewModel {
    private val _megaminxState = MutableStateFlow(MegaminxState())
    val megaminxState: StateFlow<MegaminxState> = _megaminxState.asStateFlow()

    fun swapCorners(i: Int, j: Int) {
        _megaminxState.update { state ->
            val newPositions = state.cornerPositions.toMutableList()
            val temp = newPositions[i]
            newPositions[i] = newPositions[j]
            newPositions[j] = temp
            state.copy(cornerPositions = newPositions)
        }
    }

    fun rotateCorner(index: Int, direction: Int) {
        _megaminxState.update { state ->
            val newOrientations = state.cornerOrientations.toMutableList()
            newOrientations[index] = (newOrientations[index] + direction + 3) % 3
            state.copy(cornerOrientations = newOrientations)
        }
    }

    fun swapEdges(i: Int, j: Int) {
        _megaminxState.update { state ->
            val newPositions = state.edgePositions.toMutableList()
            val temp = newPositions[i]
            newPositions[i] = newPositions[j]
            newPositions[j] = temp
            state.copy(edgePositions = newPositions)
        }
    }

    fun flipEdge(index: Int) {
        _megaminxState.update { state ->
            val newOrientations = state.edgeOrientations.toMutableList()
            newOrientations[index] = (newOrientations[index] + 1) % 2
            state.copy(edgeOrientations = newOrientations)
        }
    }

    fun reset() {
        _megaminxState.value = MegaminxState()
    }

    fun currentState(): MegaminxState = _megaminxState.value
}
